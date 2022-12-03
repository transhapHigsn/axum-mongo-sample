use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct SampleUser {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    password: String
}
