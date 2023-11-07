use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRef;
use nakago::{Inject, inject, Provider, Tag};
use nakago_derive::Provider;

/// Tag(app::State)
pub const STATE: Tag<State> = Tag::new("app::State");

/// The top-level Application State
#[derive(Clone, FromRef)]
pub struct State {}

impl nakago_axum::State for State {}

/// Provide the State for Axum
///
/// **Provides:** `app::State`
#[derive(Default)]
pub struct Provide {}

#[Provider]
#[async_trait]
impl Provider<State> for Provide {
    async fn provide(self: Arc<Self>, _i: Inject) -> inject::Result<Arc<State>> {
        Ok(Arc::new(State{}))
    }
}
