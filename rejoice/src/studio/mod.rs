//! Rejoice Studio - Visual development environment.
//!
//! This module is only active in debug builds and provides:
//! - Component registry for tracking all `#[component]` definitions
//! - Source location tracking for UI elements
//! - WebSocket API for the Studio overlay (when `--studio` flag is used)

mod file_ops;
mod protocol;
mod registry;
mod websocket;

pub use file_ops::{EditResult, FileOps};
pub use protocol::{ClientMessage, Edit, ServerMessage};
pub use registry::{
    ComponentMeta, PreviewFn, PropMeta, get_all_components, get_component, get_preview,
    register_component, register_preview, render_preview,
};
pub use websocket::{get_history_state, handle_studio_socket};
