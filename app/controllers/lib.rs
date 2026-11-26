use axum::Router;
use axum::routing::*;
use config::app::app_state::AppState;
use config::create_routes;

pub mod index;

pub fn router() -> Router<AppState> {
    //
    // Routing configuration
    // Statements take one of the forms:
    //
    // [method] [path] => [handler],
    // route [path] => [router],
    // route => [router],
    // with [middleware] => [router],
    // route [path] with [middleware] => [router],
    //
    // Note that create_routes! {...} returns a router
    //
    create_routes! {
        get "/" => index::render,
    }
}
