mod app;
pub mod codegen;
pub mod db;
pub mod env;
mod island;

pub use app::App;
pub use island::island_fn;
pub use maud::{html, Markup, DOCTYPE};
pub use serde_json::json;

/// Include the generated routes and create_router() function.
/// Call this at the top level of your main.rs.
#[macro_export]
macro_rules! routes {
    () => {
        include!(concat!(env!("OUT_DIR"), "/routes_generated.rs"));
    };
}
