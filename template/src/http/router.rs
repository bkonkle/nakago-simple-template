use axum::{routing::get, Router};
use nakago::Inject;
use nakago_axum::{init::trace_layer, State};

use super::health;

/// Initialize the HTTP router
pub fn init(i: &Inject) -> Router {
    Router::new()
        .layer(trace_layer())
        .route("/health", get(health::health_check))
        .with_state(State::new(i.clone()))
}
