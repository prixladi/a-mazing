use axum::Router;

use crate::AppState;

mod mazes;

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new().nest("/api/", mazes::routes())
}
