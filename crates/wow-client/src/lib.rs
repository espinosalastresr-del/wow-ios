//! Client application orchestration.

pub mod app;
pub mod config;

pub use app::ClientApp;
pub use config::ClientConfig;
pub mod demo;

mod gameplay;
mod content;
