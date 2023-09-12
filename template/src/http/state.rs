use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRef;
use nakago::{Inject, InjectResult, Provider, Tag};
use nakago_axum::app::State;
use nakago_derive::Provider;

/// Tag(AppState)
pub const STATE: Tag<AppState> = Tag::new("AppState");

/// The top-level Application State
#[derive(Clone, FromRef)]
pub struct AppState {}

impl State for AppState {}

/// Provide the AppState for Axum
///
/// **Provides:** `AppState`
#[derive(Default)]
pub struct ProvideAppState {}

#[Provider]
#[async_trait]
impl Provider<AppState> for ProvideAppState {
    async fn provide(self: Arc<Self>, _i: Inject) -> InjectResult<Arc<AppState>> {
        Ok(Arc::new(AppState{}))
    }
}
