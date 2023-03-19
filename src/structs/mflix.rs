use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct SampleUser {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
//    #[serde(skip_deserializing)]
//    password: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub data: Option<Vec<SampleUser>>,
    pub error_message: Option<String>
}

#[skip_serializing_none]
#[derive(Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
    pub sort_by: Option<String>,
    pub order: Option<String>
}

impl Pagination {
    pub fn check(&self) -> Result<(), String> {
        if self.page < 1 {
            return Err("Page must be greater than or equal to 1.".into());
        }

        if self.per_page < 1 {
            return Err("Rows per page must be greater than or equal to 1.".into());
        } else if self.per_page > 100 {
            return Err("Rows per page must be less than or equal to 100.".into());
        }

        if let Some(val) = &self.sort_by {
            if !(["_id", "name", "email"].contains(&val.as_str())) {
                return Err("Invalid value passed for sort_by query parameter. Must be one of: _id, email or name.".into());
            }

            if let Some(ord) = &self.order {
                if !(["asc", "desc"]).contains(&ord.as_str()) {
                    return Err("Invalid value passed for order query parameter. Must be one of: asc or desc.".into());
                }
            };
        }
        Ok(())
    }
}