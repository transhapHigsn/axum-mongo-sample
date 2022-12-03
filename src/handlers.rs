use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use futures::stream::StreamExt;
use mongodb::{Client, Collection, bson::doc, options::FindOptions};

use crate::structs::{CreateUser, SampleUser, User};

// basic handler that responds with a static string
pub async fn root(State(_client): State<Client>) -> &'static str {
    "Hello, World! from Axum"
}

pub async fn create_user(
        State(_client): State<Client>,
        // this argument tells axum to parse the request body
        // as JSON into a `CreateUser` type
        Json(payload): Json<CreateUser>,
    ) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

pub async fn sample_users(State(client): State<Client>) -> impl IntoResponse {
    let users_coll: Collection<SampleUser> = client.database("sample_mflix").collection::<SampleUser>("users");
    let mut options = FindOptions::default();
    options.limit = Some(10);
    options.sort = Some(doc! {
        "name": 1
    });
    let mut users_cursor = users_coll.find(None, options).await.expect("could not load users data.");
    let mut users: Vec<SampleUser> = Vec::new();

    while let Some(doc) = users_cursor.next().await {
        users.push(doc.expect("could not load user info."));
    }

    (StatusCode::OK, Json(users))

}