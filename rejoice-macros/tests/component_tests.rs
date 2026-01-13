//! Tests for the #[component] macro

use maud::{Render, html};
use rejoice_macros::component;

// =============================================================================
// Basic component tests
// =============================================================================

/// Test that a simple component with no props compiles and renders
#[component]
pub fn Empty() -> Markup {
    html! { div { "Empty" } }
}

#[test]
fn test_empty_component() {
    let rendered = Empty::new().render().into_string();
    assert!(rendered.contains("Empty"));
    assert!(rendered.contains("<div>"));
}

/// Test component with a single required prop
#[component]
pub fn SingleProp(name: &str) -> Markup {
    html! { span { "Hello, " (name) } }
}

#[test]
fn test_single_prop_component() {
    let rendered = SingleProp::new("World").render().into_string();
    assert!(rendered.contains("Hello, World"));
}

/// Test component with multiple required props
#[component]
pub fn MultipleRequired(first: &str, second: &str) -> Markup {
    html! { p { (first) " and " (second) } }
}

#[test]
fn test_multiple_required_props() {
    let rendered = MultipleRequired::new("Alice", "Bob").render().into_string();
    assert!(rendered.contains("Alice and Bob"));
}

// =============================================================================
// Default value tests
// =============================================================================

/// Test component with explicit default value
#[component]
pub fn WithExplicitDefault(label: &str, #[prop(default = 42)] count: i32) -> Markup {
    html! { div { (label) ": " (count) } }
}

#[test]
fn test_explicit_default_uses_default() {
    let rendered = WithExplicitDefault::new("Count").render().into_string();
    assert!(rendered.contains("Count: 42"));
}

#[test]
fn test_explicit_default_can_be_overridden() {
    let rendered = WithExplicitDefault::new("Count")
        .count(100)
        .render()
        .into_string();
    assert!(rendered.contains("Count: 100"));
}

/// Test component with Default trait default
#[component]
pub fn WithTraitDefault(#[prop(default)] enabled: bool) -> Markup {
    html! {
        @if enabled {
            "Enabled"
        } @else {
            "Disabled"
        }
    }
}

#[test]
fn test_trait_default_uses_default() {
    // bool::default() is false
    let rendered = WithTraitDefault::new().render().into_string();
    assert!(rendered.contains("Disabled"));
}

#[test]
fn test_trait_default_can_be_overridden() {
    let rendered = WithTraitDefault::new().enabled(true).render().into_string();
    assert!(rendered.contains("Enabled"));
}

// =============================================================================
// Option type tests
// =============================================================================

/// Test component with Option prop (auto-defaults to None)
#[component]
pub fn WithOption(title: &str, subtitle: Option<&str>) -> Markup {
    html! {
        h1 { (title) }
        @if let Some(sub) = subtitle {
            p { (sub) }
        }
    }
}

#[test]
fn test_option_defaults_to_none() {
    let rendered = WithOption::new("Hello").render().into_string();
    assert!(rendered.contains("<h1>Hello</h1>"));
    assert!(!rendered.contains("<p>"));
}

#[test]
fn test_option_can_be_set() {
    let rendered = WithOption::new("Hello")
        .subtitle(Some("World"))
        .render()
        .into_string();
    assert!(rendered.contains("<h1>Hello</h1>"));
    assert!(rendered.contains("<p>World</p>"));
}

// =============================================================================
// Builder pattern tests
// =============================================================================

/// Test that builder methods chain correctly
#[component]
pub fn BuilderChain(
    #[prop(default = "default")] a: &str,
    #[prop(default = "default")] b: &str,
    #[prop(default = "default")] c: &str,
) -> Markup {
    html! { (a) "-" (b) "-" (c) }
}

#[test]
fn test_builder_chain() {
    let rendered = BuilderChain::new()
        .a("first")
        .b("second")
        .c("third")
        .render()
        .into_string();
    assert!(rendered.contains("first-second-third"));
}

#[test]
fn test_partial_builder_chain() {
    let rendered = BuilderChain::new().b("middle").render().into_string();
    assert!(rendered.contains("default-middle-default"));
}

// =============================================================================
// Type tests
// =============================================================================

/// Test component with non-reference types
#[component]
pub fn NonRefTypes(
    count: i32,
    #[prop(default = 3.14)] pi: f64,
    #[prop(default = true)] flag: bool,
) -> Markup {
    html! {
        div { "count=" (count) " pi=" (pi) " flag=" (flag) }
    }
}

#[test]
fn test_non_ref_types() {
    let rendered = NonRefTypes::new(5).render().into_string();
    assert!(rendered.contains("count=5"));
    assert!(rendered.contains("pi=3.14"));
    assert!(rendered.contains("flag=true"));
}

// =============================================================================
// Maud integration tests
// =============================================================================

/// Test that components work inside html! macro
#[component]
pub fn Inner(text: &str) -> Markup {
    html! { em { (text) } }
}

#[test]
fn test_component_in_html_macro() {
    let rendered = html! {
        div {
            (Inner::new("nested"))
        }
    }
    .into_string();
    // In debug mode, components are wrapped with data-component divs
    assert!(rendered.contains("<em>nested</em>"));
    assert!(rendered.contains("<div>"));
    // Debug mode adds data-component attribute
    #[cfg(debug_assertions)]
    assert!(rendered.contains("data-component=\"Inner\""));
}

/// Test nested components
#[component]
pub fn Outer(#[prop(default = "outer")] label: &str) -> Markup {
    html! {
        section {
            (label)
            (Inner::new("inner"))
        }
    }
}

#[test]
fn test_nested_components() {
    let rendered = Outer::new().render().into_string();
    assert!(rendered.contains("outer"));
    assert!(rendered.contains("<em>inner</em>"));
}

// =============================================================================
// Children prop tests
// =============================================================================

use maud::Markup;

/// Test component with children prop
#[component]
pub fn Card(title: &str, #[prop(default = html!{})] children: Markup) -> Markup {
    html! {
        div class="card" {
            h2 { (title) }
            div class="content" { (children) }
        }
    }
}

#[test]
fn test_children_default_empty() {
    let rendered = Card::new("My Card").render().into_string();
    assert!(rendered.contains("<h2>My Card</h2>"));
    assert!(rendered.contains("<div class=\"content\"></div>"));
}

#[test]
fn test_children_with_content() {
    let rendered = Card::new("My Card")
        .children(html! { p { "Hello" } })
        .render()
        .into_string();
    assert!(rendered.contains("<h2>My Card</h2>"));
    assert!(rendered.contains("<p>Hello</p>"));
}

#[test]
fn test_children_with_nested_components() {
    let rendered = Card::new("My Card")
        .children(html! {
            (Inner::new("nested child"))
        })
        .render()
        .into_string();
    assert!(rendered.contains("<h2>My Card</h2>"));
    assert!(rendered.contains("<em>nested child</em>"));
}

/// Test component with only children (wrapper component)
#[component]
pub fn Wrapper(#[prop(default = html!{})] children: Markup) -> Markup {
    html! {
        div class="wrapper" { (children) }
    }
}

#[test]
fn test_wrapper_component() {
    let rendered = Wrapper::new()
        .children(html! { span { "wrapped" } })
        .render()
        .into_string();
    assert!(rendered.contains("<div class=\"wrapper\"><span>wrapped</span></div>"));
}

/// Test deeply nested children
#[test]
fn test_deeply_nested_children() {
    let rendered = Wrapper::new()
        .children(html! {
            (Card::new("Nested Card").children(html! {
                p { "Deep content" }
            }))
        })
        .render()
        .into_string();
    assert!(rendered.contains("wrapper"));
    assert!(rendered.contains("Nested Card"));
    assert!(rendered.contains("Deep content"));
}
