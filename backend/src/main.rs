use backend::DbClient;
use dotenv::dotenv;
use std::env;

extern crate diesel;
extern crate tracing;

mod backend;
mod models;
mod routes;
mod schema;
mod storage;

#[derive(Clone)]
pub struct AppState {
    pub db: DbClient,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let db_client = backend::DbClient::new(&database_url);

    let app_state = AppState { db: db_client };

    let app = routes::create_router(app_state);
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}
