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
use mongodb::{Client, options::{Compressor, ClientOptions}};
use tower_http::{trace::TraceLayer, set_header::SetResponseHeaderLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use handlers::{root, create_user, handler_404, sample_users};

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

    let mongo_uri: String = std::env::var("MONGO_URI").expect("Failed to load `MONGO_MAX_POOL_SIZE` environment variable.");
    let mongo_connection_timeout: u64 = std::env::var("MONGO_CONNECTION_TIMEOUT")
        .expect("Failed to load `MONGO_CONNECTION_TIMEOUT` environment variable.")
        .parse()
        .expect("Failed to parse `MONGO_CONNECTION_TIMEOUT` environment variable.");
    let mongo_min_pool_size: u32 = std::env::var("MONGO_MIN_POOL_SIZE")
        .expect("Failed to load `MONGO_MIN_POOL_SIZE` environment variable.")
        .parse()
        .expect("Failed to parse `MONGO_MIN_POOL_SIZE` environment variable.");
    let mongo_max_pool_size: u32 = std::env::var("MONGO_MAX_POOL_SIZE")
        .expect("Failed to load `MONGO_MAX_POOL_SIZE` environment variable.")
        .parse()
        .expect("Failed to parse `MONGO_MAX_POOL_SIZE` environment variable.");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "rust_axum=debug,axum=debug,tower_http=debug,mongodb=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    client_options.connect_timeout = Some(Duration::from_secs(mongo_connection_timeout));
    client_options.max_pool_size = Some(mongo_max_pool_size);
    client_options.min_pool_size = Some(mongo_min_pool_size);

    // the server will select the algorithm it supports from the list provided by the driver
    client_options.compressors = Some(vec![
        Compressor::Snappy,
        Compressor::Zlib {
            level: Default::default(),
        },
        Compressor::Zstd {
            level: Default::default(),
        },
    ]);
    let client = Client::with_options(client_options).unwrap();
    let server_header_value = HeaderValue::from_static("rust-axum");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/sample_users", get(sample_users))
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::SERVER,
            server_header_value,
        ));
    let app = app.fallback(handler_404).with_state(client);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
