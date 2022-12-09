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
