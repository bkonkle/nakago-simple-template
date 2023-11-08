use nakago::{config, inject, EventType};
use nakago_axum::{
    config::default_loaders,
    AxumApplication,
};

use crate::{
    config::{Config, CONFIG},
    http,
};

/// Create a default AxumApplication instance
pub async fn app() -> inject::Result<AxumApplication<Config, State>> {
    let mut app = AxumApplication::default().with_config_tag(&CONFIG);

    app.on(&EventType::Load, config::AddLoaders::new(default_loaders()));

    app.on(&EventType::Init, http::Init::default());

    Ok(app)
}
