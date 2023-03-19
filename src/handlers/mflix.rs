use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json
};
use futures::stream::StreamExt;
use mongodb::{
    bson::{Bson, doc, Document, oid::ObjectId},
    Client,
    Collection,
    options::{FindOptions, FindOneOptions},
};

use crate::structs::mflix::{Pagination, Response, SampleUser};


pub async fn list_users(State(client): State<Client>, pagination: Query<Pagination>) -> impl IntoResponse {
    if let Err(message) = pagination.check() {
        let response = Response {
            success: false,
            data: None,
            error_message: Some(message)
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    let order: i64 = match pagination.order.as_str() {
        "desc" => -1,
        _ => 1,
    };
    let sort_by: bson::Document = doc! {&pagination.sort_by: order};

    let users_coll: Collection<SampleUser> = client
        .database("sample_mflix")
        .collection::<SampleUser>("users");
    let mut options = FindOptions::default();
    options.limit = Some(pagination.per_page as i64);
    options.skip = Some((pagination.page as u64 - 1) * pagination.per_page as u64);
    options.sort = Some(sort_by);
    options.projection = Some(doc! {
        "name": 1,
        "email": 1
    });
    let mut users_cursor = users_coll
        .find(None, options)
        .await
        .expect("could not load users data.");
    let mut users: Vec<SampleUser> = Vec::new();
    while let Some(doc) = users_cursor.next().await {
        users.push(doc.expect("could not load user info."));
    }

    let response = Response {
        success: true,
        data: Some(users),
        error_message: None
    };
    (StatusCode::OK, Json(response))
}

pub async fn user_by_id(State(client): State<Client>, user_id: Path<String>) -> impl IntoResponse {
    let id = ObjectId::parse_str(user_id.0);
    if let Err(err) = id {
        return (StatusCode::BAD_REQUEST, Json(Response {
            success: false,
            error_message: Some(format!("Invalid value provided for id, reason: {:#?}", err)),
            data: None
        }));
    }

    fetch_user(client, doc! {
        "_id": id.unwrap()
    }).await
}

pub async fn user_by_name(State(client): State<Client>, name: Path<String>) -> impl IntoResponse {
    let user_name = name.0;
    fetch_user(client, doc! {
        "name": &user_name
    }).await
}


pub async fn user_by_email(State(client): State<Client>, email: Path<String>) -> impl IntoResponse {
    let user_email = email.0;
    fetch_user(client, doc! {
        "email": &user_email
    }).await
}


async fn fetch_user(client: Client, filter: Document) -> (StatusCode, Json<Response>) {
    let users_coll: Collection<SampleUser> = client
        .database("sample_mflix")
        .collection::<SampleUser>("users");

    let mut options = FindOneOptions::default();
    options.projection = Some(doc! {
        "name": 1,
        "email": 1
    });

    let user = users_coll.find_one(filter.clone(), options).await;
    match user {
        Ok(value) => {
            match value {
                Some(user) => {
                    (StatusCode::FOUND, Json(Response {
                        success: true,
                        data: Some(vec![user]),
                        error_message: None
                    }))
                },
                None => {
                    let mut message: String = "".to_owned();
                    for (k, v) in filter {
                        let message_part = match v {
                            Bson::String(val) => format!("{}=={}, ", k, val),
                            _ => format!("{}=={}, ", k, v)
                        };
                        message.push_str(&message_part);
                    }
                    (StatusCode::NOT_FOUND, Json(Response {
                        success: false,
                        error_message: Some(format!("No user exists for given filter: {}", message)),
                        data: None
                    }))
                }
            }
        },
        Err(err) => {
            (StatusCode::NOT_FOUND, Json(Response {
                success: false,
                error_message: Some(format!("Couldn't find any user due to {:#?}", err)),
                data: None
            }))
        }
    }
}