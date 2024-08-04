use axum::Router;

use crate::AppState;

mod generate;
mod get_by_id;

pub fn routes() -> Router<AppState> {
    Router::<AppState>::new().nest("/mazes/", generate::routes())
}
