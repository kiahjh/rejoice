//! Component registry for Rejoice Studio.
//!
//! This module provides a global registry of component metadata that is populated
//! at runtime when components are first rendered. Only active in debug builds.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// Metadata about a single prop on a component.
#[derive(Debug, Clone)]
pub struct PropMeta {
    /// The prop name (e.g., "label", "size")
    pub name: &'static str,
    /// The type as a string (e.g., "&str", "ButtonSize", "bool")
    pub ty: &'static str,
    /// Whether this prop is required (passed to `new()`)
    pub required: bool,
    /// The default value as a string, if optional (e.g., "false", "ButtonSize::Medium")
    pub default: Option<&'static str>,
    /// Doc comment for this prop, if any
    pub doc: Option<&'static str>,
}

/// Metadata about a component.
#[derive(Debug, Clone)]
pub struct ComponentMeta {
    /// The component name (e.g., "Button", "Card")
    pub name: &'static str,
    /// Source file where the component is defined
    pub file: &'static str,
    /// Line number in the source file
    pub line: u32,
    /// Column number in the source file
    pub column: u32,
    /// Doc comment for the component, if any
    pub doc: Option<&'static str>,
    /// Props metadata
    pub props: &'static [PropMeta],
}

/// Global component registry.
static REGISTRY: OnceLock<RwLock<HashMap<&'static str, ComponentMeta>>> = OnceLock::new();

fn get_registry() -> &'static RwLock<HashMap<&'static str, ComponentMeta>> {
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Register a component in the global registry.
///
/// This is called automatically by the `#[component]` macro in debug builds.
/// If a component with the same name is already registered, it will be replaced.
pub fn register_component(meta: ComponentMeta) {
    if let Ok(mut registry) = get_registry().write() {
        registry.insert(meta.name, meta);
    }
}

/// Get a component's metadata by name.
pub fn get_component(name: &str) -> Option<ComponentMeta> {
    get_registry().read().ok()?.get(name).cloned()
}

/// Get all registered components.
pub fn get_all_components() -> Vec<ComponentMeta> {
    get_registry()
        .read()
        .ok()
        .map(|r| r.values().cloned().collect())
        .unwrap_or_default()
}

/// Clear all registered components (useful for testing).
#[cfg(test)]
pub fn clear_registry() {
    if let Ok(mut registry) = get_registry().write() {
        registry.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_get_component() {
        clear_registry();

        let meta = ComponentMeta {
            name: "TestButton",
            file: "src/components.rs",
            line: 42,
            column: 1,
            doc: Some("A test button component"),
            props: &[
                PropMeta {
                    name: "label",
                    ty: "&str",
                    required: true,
                    default: None,
                    doc: Some("The button text"),
                },
                PropMeta {
                    name: "disabled",
                    ty: "bool",
                    required: false,
                    default: Some("false"),
                    doc: None,
                },
            ],
        };

        register_component(meta);

        let retrieved = get_component("TestButton").expect("Component should be registered");
        assert_eq!(retrieved.name, "TestButton");
        assert_eq!(retrieved.file, "src/components.rs");
        assert_eq!(retrieved.line, 42);
        assert_eq!(retrieved.props.len(), 2);
        assert_eq!(retrieved.props[0].name, "label");
        assert!(retrieved.props[0].required);
        assert_eq!(retrieved.props[1].name, "disabled");
        assert!(!retrieved.props[1].required);
    }

    #[test]
    fn test_get_all_components() {
        clear_registry();

        register_component(ComponentMeta {
            name: "ComponentA",
            file: "a.rs",
            line: 1,
            column: 1,
            doc: None,
            props: &[],
        });

        register_component(ComponentMeta {
            name: "ComponentB",
            file: "b.rs",
            line: 1,
            column: 1,
            doc: None,
            props: &[],
        });

        let all = get_all_components();
        assert_eq!(all.len(), 2);

        let names: Vec<_> = all.iter().map(|c| c.name).collect();
        assert!(names.contains(&"ComponentA"));
        assert!(names.contains(&"ComponentB"));
    }

    #[test]
    fn test_get_nonexistent_component() {
        clear_registry();
        assert!(get_component("NonExistent").is_none());
    }
}
