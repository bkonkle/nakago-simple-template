use nakago::{inject, EventType};
use nakago_axum::{config, AxumApplication};

use crate::{
    config::{Config, CONFIG},
    http,
};

/// Create a default AxumApplication instance
pub async fn app() -> inject::Result<AxumApplication<Config>> {
    let mut app = AxumApplication::default().with_config_tag(&CONFIG);

    app.on(&EventType::Load, config::AddLoaders::default());

    app.on(&EventType::Init, http::Init::default());

    Ok(app)
}
