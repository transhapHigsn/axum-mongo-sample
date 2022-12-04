use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use futures::stream::StreamExt;
use mongodb::{bson::doc, options::FindOptions, Client, Collection};

use crate::structs::airbnb::ListingAndReview;

pub async fn listings_and_reviews(State(client): State<Client>) -> impl IntoResponse {
    let coll: Collection<ListingAndReview> = client
        .database("sample_airbnb")
        .collection::<ListingAndReview>("listingsAndReviews");
    let mut options = FindOptions::default();
    options.limit = Some(10);
    options.sort = Some(doc! {
        "name": 1
    });
    let mut cursor = coll
        .find(None, options)
        .await
        .expect("could not load listings data.");
    let mut rows: Vec<ListingAndReview> = Vec::new();

    while let Some(doc) = cursor.next().await {
        rows.push(doc.expect("could not load listings info."));
    }

    (StatusCode::OK, Json(rows))
}
