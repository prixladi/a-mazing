use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use sea_orm::*;
use serde::Deserialize;
use validator::Validate;

use crate::AppState;

use crate::entities::maze;

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new().route("/generate", post(generate_maze))
}

#[derive(Debug, Validate, Deserialize)]
struct CreateMazeDto {
    
}


async fn generate_maze(state: State<AppState>) -> Result<Json<()>, (StatusCode, String)> {
    let maz = maze::ActiveModel {
        ..Default::default()
    };

    maz.save(&state.conn).await;

    Ok(Json(()))
}
