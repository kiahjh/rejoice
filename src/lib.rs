mod app;
pub mod codegen;
pub mod db;
pub mod env;
pub mod html;
mod island;

pub use app::App;
pub use axum::extract::{Path, State};
pub use island::island_fn;
use maud::Markup;
pub use serde_json::json;



/// Children passed to a layout component.
/// Use this in your layout function signature and render with `(children)` in Maud.
pub type Children = Markup;

/// Include the generated routes and create_router() function.
/// Call this at the top level of your main.rs.
///
/// Usage:
/// - `routes!()` - for apps without state
/// - `routes!(AppState)` - for apps with custom state type
#[macro_export]
macro_rules! routes {
    () => {
        type __RejoiceState = ();
        include!(concat!(env!("OUT_DIR"), "/routes_generated.rs"));
    };
    ($state:ty) => {
        type __RejoiceState = $state;
        include!(concat!(env!("OUT_DIR"), "/routes_generated.rs"));
    };
}
