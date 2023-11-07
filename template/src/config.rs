use axum::extract::FromRef;
use nakago::Tag;
use serde::Serialize;
use serde_derive::Deserialize;

/// Tag(app::Config)
pub const CONFIG: Tag<Config> = Tag::new("app::Config");

/// Server Config
#[derive(Default, Debug, Serialize, Deserialize, Clone, FromRef)]
pub struct Config {
    /// HTTP config
    pub http: nakago_axum::Config,
}

impl nakago::Config for Config {}
