use async_trait::async_trait;
use axum::{routing::get, Router};
use nakago::{hooks, Hook, Inject};
use nakago_axum::routes;

use super::health;

/// Init all handlers
#[derive(Default)]
pub struct Init {}

#[async_trait]
impl Hook for Init {
    async fn handle(&self, i: Inject) -> hooks::Result<()> {
        i.handle(routes::Init::new(
            Router::new().route("/health", get(health::health_check)),
        ))
        .await?;

        Ok(())
    }
}
