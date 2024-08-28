use nakago_derive::FromRef;
use serde::Serialize;
use serde_derive::Deserialize;

/// Server Config
#[derive(Default, Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct Config {
    /// HTTP config
    pub http: nakago_axum::Config,
}

impl nakago_figment::Config for Config {}
