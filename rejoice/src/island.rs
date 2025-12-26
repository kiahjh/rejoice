use maud::{Markup, PreEscaped};

/// Renders an island placeholder that will be hydrated on the client.
/// 
/// # Example
/// ```rust
/// use rejoice::{html, island, Markup};
///
/// pub async fn handler() -> Markup {
///     html! {
///         h1 { "My Page" }
///         (island!(Counter, { initial: 5 }))
///     }
/// }
/// ```
#[macro_export]
macro_rules! island {
    ($name:ident) => {
        $crate::island_fn(stringify!($name), $crate::json!({}))
    };
    ($name:ident, { $($json:tt)* }) => {
        $crate::island_fn(stringify!($name), $crate::json!({ $($json)* }))
    };
}

#[doc(hidden)]
pub fn island_fn(name: &str, props: serde_json::Value) -> Markup {
    let props_json = serde_json::to_string(&props).unwrap_or_else(|_| "{}".to_string());
    let escaped_props = html_escape::encode_double_quoted_attribute(&props_json);

    PreEscaped(format!(
        r#"<div data-island="{}" data-props="{}"></div>"#,
        name, escaped_props
    ))
}


