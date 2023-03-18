mod handlers;
mod structs;

//use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use axum::{
    http::{header, HeaderValue},
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use mongodb::{
    options::ClientOptions,
    Client,
};
use tower_http::{
    limit::RequestBodyLimitLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use handlers::{
    airbnb::listings_and_reviews,
    common::{handler_404, root},
    mflix::{list_users, user_by_email, user_by_id, user_by_name},
    sample::create_user,
};
use structs::common::DatabaseConfig;

//#[derive(Debug)]
//pub enum MongoError {
//    Io(io::Error),
//    Mongo(mongodb::error::Error),
//}
//
//impl From<io::Error> for MongoError {
//    fn from(err: io::Error) -> Self {
//        MongoError::Io(err)
//    }
//}
//
//impl From<mongodb::error::Error> for MongoError {
//    fn from(err: mongodb::error::Error) -> Self {
//        MongoError::Mongo(err)
//    }
//}

#[tokio::main]
async fn main() {
    // initialize tracing
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "rust_axum=debug,axum=debug,tower_http=debug,mongodb=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_config = DatabaseConfig::new();
    let mut client_options = ClientOptions::parse(database_config.uri).await.unwrap();
    client_options.connect_timeout = database_config.connection_timeout;
    client_options.max_pool_size = database_config.max_pool_size;
    client_options.min_pool_size = database_config.min_pool_size;
    // the server will select the algorithm it supports from the list provided by the driver
    client_options.compressors = database_config.compressors;
    let client = Client::with_options(client_options).unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/sample/users/", post(create_user))
        .route("/mflix/users/", get(list_users))
        .route("/mflix/user/id/:id/", get(user_by_id))
        .route("/mflix/user/name/:name/", get(user_by_name))
        .route("/mflix/user/email/:email/", get(user_by_email))
        .route("/airbnb/listings_and_reviews/", get(listings_and_reviews))
        // timeout requests after 10 secs, returning 408 status code
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        // don't allow request bodies larger than 1024 bytes, returning 413 status code
        .layer(RequestBodyLimitLayer::new(1024))
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::SERVER,
            HeaderValue::from_static("rust-axum"),
        ));
    let app = app.fallback(handler_404).with_state(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
