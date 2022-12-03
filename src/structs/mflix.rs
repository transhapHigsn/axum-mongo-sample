use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct SampleUser {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    password: String,
}
