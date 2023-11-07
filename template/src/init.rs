use nakago::{config, inject, EventType};
use nakago_axum::{config::default_loaders, routes, AxumApplication};

use crate::{
    config::{Config, CONFIG},
    http::{
        routes::new_health_route,
        state::{self, State, STATE},
    },
};

/// Create a default AxumApplication instance
pub async fn app() -> inject::Result<AxumApplication<Config, State>> {
    let mut app = AxumApplication::default()
        .with_config_tag(&CONFIG)
        .with_state_tag(&STATE);

    // Dependencies

    app.provide(&STATE, state::Provide::default()).await?;

    // Config

    app.on(&EventType::Load, config::AddLoaders::new(default_loaders()));

    // Routes

    app.on(&EventType::Init, routes::Init::new(new_health_route));

    Ok(app)
}
