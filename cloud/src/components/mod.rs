//! UI Component System for Rejoice Cloud
//!
//! This module provides a cohesive, reusable component library built on Tailwind CSS.
//! All UI elements should be built using these components for consistency.
//!
//! # Design Principles
//! - Components are functions that return `Markup`
//! - Use Tailwind classes exclusively (no inline styles)
//! - Components are composable and accept children where appropriate
//! - Variants are handled via enums, not string parameters

mod button;
mod card;
mod form;
mod layout;
mod typography;

// Icons are in their own public submodule to avoid naming conflicts
pub mod icon;

pub use button::*;
pub use card::*;
pub use form::*;
pub use layout::*;
pub use typography::*;
