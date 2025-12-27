mod app;
pub mod codegen;
pub mod db;
pub mod env;
mod island;

pub use app::App;
pub use island::island_fn;
pub use maud::{html, Markup, PreEscaped, DOCTYPE};
pub use serde_json::json;

/// Children passed to a layout component.
/// Use this in your layout function signature and render with `(children)` in Maud.
pub type Children = Markup;

/// Include the generated routes and create_router() function.
/// Call this at the top level of your main.rs.
#[macro_export]
macro_rules! routes {
    () => {
        include!(concat!(env!("OUT_DIR"), "/routes_generated.rs"));
    };
}
