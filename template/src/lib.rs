//! # A simple server
#![forbid(unsafe_code)]

/// HTTP entry points
pub mod http;

/// Application config
pub mod config;

/// App initialization
pub mod init;

pub use config::Config;
