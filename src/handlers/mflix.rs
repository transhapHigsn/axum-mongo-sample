use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};

use futures::stream::StreamExt;
use mongodb::{bson::doc, options::FindOptions, Client, Collection};

use crate::structs::mflix::{Pagination, Response, SampleUser};


pub async fn sample_users(State(client): State<Client>, pagination: Query<Pagination>) -> impl IntoResponse {
    if pagination.page < 1 {
        let response = Response {
            success: false,
            data: None,
            error_message: Some("Page must be greater than or equal to 1.".into())
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    let sort_by: bson::Document;
    let mut order: i64 = 1;
    if let Some(val) = &pagination.sort_by {
        if !(["_id", "name", "email"].contains(&val.as_str())) {
            let response = Response {
                success: false,
                data: None,
                error_message: Some("Invalid value passed for sort_by query parameter. Must be one of: _id, email or name.".into())
            };
            return (StatusCode::BAD_REQUEST, Json(response));
        }

        if let Some(ord) = &pagination.order {
            if !(["asc", "desc"]).contains(&ord.as_str()) {
                let response = Response {
                    success: false,
                    data: None,
                    error_message: Some("Invalid value passed for order query parameter. Must be one of: asc or desc.".into())
                };
                return (StatusCode::BAD_REQUEST, Json(response));
            }

            if ord == "desc" {
                order = -1;
            }
        };

        sort_by = doc! {
            val: order
        };
    } else {
        sort_by = doc! {
            "name": order
        };
    };

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
