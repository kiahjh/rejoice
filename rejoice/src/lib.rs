mod app;
pub mod codegen;
#[cfg(feature = "sqlite")]
pub mod db;
pub mod env;
mod island;
mod request;
mod response;

// Re-export dotenvy for the env! macro
#[doc(hidden)]
pub use dotenvy_macro;

// Core types
pub use app::App;
pub use request::Req;
pub use response::Res;

// Axum extractors that are still useful
pub use axum::extract::Path;

// Re-export axum types needed by generated code
#[doc(hidden)]
pub use axum::extract::State;
#[doc(hidden)]
pub use axum::{Router, routing};

// Island support
pub use island::island_fn;

// HTML/Maud - flattened to root level
pub use maud::{html, Markup, DOCTYPE, PreEscaped};

// JSON
pub use serde_json::json;

// Type alias for layout children
pub type Children = Markup;

/// Read an environment variable at compile time from .env file.
/// Re-exports dotenvy_macro::dotenv.
#[macro_export]
macro_rules! env {
    ($key:expr) => {
        $crate::dotenvy_macro::dotenv!($key)
    };
}

/// Marker type for stateless apps.
/// This is distinct from `()` to avoid trait impl conflicts.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct NoState;

/// Include the generated routes and create_router() function.
/// Call this at the top level of your main.rs.
///
/// Usage:
/// - `routes!()` - for apps without state
/// - `routes!(AppState)` - for apps with custom state type
#[macro_export]
macro_rules! routes {
    () => {
        #[allow(dead_code)]
        type __RejoiceState = rejoice::NoState;
        include!(concat!(env!("OUT_DIR"), "/routes_generated_stateless.rs"));
    };
    ($state:ty) => {
        #[allow(dead_code)]
        type __RejoiceState = $state;
        include!(concat!(env!("OUT_DIR"), "/routes_generated_stateful.rs"));
    };
}

/// Prelude module for convenient imports.
///
/// ```rust
/// use rejoice::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        html, island, json, App, Children, Markup, Path, PreEscaped, Req, Res, DOCTYPE,
    };
}


