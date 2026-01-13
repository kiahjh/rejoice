//! Test that components without return type produce an error

use rejoice_macros::component;

#[component]
pub fn NoReturn() {
    // Missing -> Markup
}

fn main() {}
