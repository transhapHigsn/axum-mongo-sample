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
    pub page: i32,
    pub per_page: i32,
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
    #[serde(default = "default_order")]
    pub order: String
}

fn default_sort_by() -> String {
    "name".to_string()
}

fn default_order() -> String {
    "asc".to_string()
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

        if !(["_id".to_string(), "name".to_string(), "email".to_string()].contains(&self.sort_by)) {
            return Err("Invalid value passed for sort_by query parameter. Must be one of: _id, email or name.".into());
        }

        if !(["asc".to_string(), "desc".to_string()]).contains(&self.order) {
            return Err("Invalid value passed for order query parameter. Must be one of: asc or desc.".into());
        }
        Ok(())
    }
}