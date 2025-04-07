use axum::Router;
use axum::routing::*;
use config::app::app_state::AppState;
use config::create_routes;

pub mod index;

pub fn router() -> Router<AppState> {
    create_routes! {
        get "/" => index::render,
    }
}
