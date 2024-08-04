use std::error::Error;

use axum::serve;
use routes::routes;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;

use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod entities;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let conn = Database::connect("postgres://default:FHD7nyLZ1rVO@ep-bitter-hill-a2zytdwe.eu-central-1.aws.neon.tech:5432/verceldb?sslmode=require")
        .await
        .expect("Database connection failed");

    let state = AppState { conn };

    let router = routes().with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    info!("listening on {}", listener.local_addr()?);

    serve(listener, router).await?;

    Ok(())
}

#[derive(Clone)]
pub(crate) struct AppState {
    conn: DatabaseConnection,
}
