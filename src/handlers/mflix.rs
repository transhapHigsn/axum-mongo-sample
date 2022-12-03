use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use futures::stream::StreamExt;
use mongodb::{bson::doc, options::FindOptions, Client, Collection};

use crate::structs::mflix::SampleUser;

pub async fn sample_users(State(client): State<Client>) -> impl IntoResponse {
    let users_coll: Collection<SampleUser> = client
        .database("sample_mflix")
        .collection::<SampleUser>("users");
    let mut options = FindOptions::default();
    options.limit = Some(10);
    options.sort = Some(doc! {
        "name": 1
    });
    let mut users_cursor = users_coll
        .find(None, options)
        .await
        .expect("could not load users data.");
    let mut users: Vec<SampleUser> = Vec::new();

    while let Some(doc) = users_cursor.next().await {
        users.push(doc.expect("could not load user info."));
    }

    (StatusCode::OK, Json(users))
}
