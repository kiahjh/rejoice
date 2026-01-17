use colored::Colorize;
use std::fs;
use std::path::Path;

/// Check if the app uses state by looking for AppState in main.rs
fn app_uses_state() -> bool {
    let main_rs = Path::new("src/main.rs");
    if let Ok(content) = fs::read_to_string(main_rs) {
        // Look for common patterns that indicate stateful app
        content.contains("AppState") || content.contains("App::with_state")
    } else {
        false
    }
}

/// Extract dynamic parameter names from the file path
/// e.g., "src/routes/users/[id]/posts/[post_id]/index.rs" -> vec!["id", "post_id"]
fn extract_params_from_path(path: &Path) -> Vec<String> {
    let mut params = Vec::new();

    for component in path.components() {
        let name = component.as_os_str().to_string_lossy();
        if name.starts_with('[') && name.ends_with(']') {
            let param = &name[1..name.len() - 1];
            params.push(param.to_string());
        }
    }

    // Also check the file name itself (without .rs extension)
    if let Some(stem) = path.file_stem() {
        let stem = stem.to_string_lossy();
        if stem.starts_with('[') && stem.ends_with(']') {
            let param = &stem[1..stem.len() - 1];
            params.push(param.to_string());
        }
    }

    params
}

/// Generate boilerplate for a layout file
fn generate_layout_boilerplate(uses_state: bool) -> String {
    if uses_state {
        r#"use crate::AppState;
use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(state: AppState, req: Req, res: Res, children: Children) -> Res {
    let _ = (state, req); // Silence unused warnings

    res.html(html! {
        (DOCTYPE)
        html {
            head {
                title { "Page Title" }
            }
            body {
                (children)
            }
        }
    })
}
"#
        .to_string()
    } else {
        r#"use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(req: Req, res: Res, children: Children) -> Res {
    let _ = req; // Silence unused warning

    res.html(html! {
        (DOCTYPE)
        html {
            head {
                title { "Page Title" }
            }
            body {
                (children)
            }
        }
    })
}
"#
        .to_string()
    }
}

/// Generate boilerplate for a route file
fn generate_route_boilerplate(uses_state: bool, params: &[String]) -> String {
    let params_str = params.join(", ");
    let params_typed = params
        .iter()
        .map(|p| format!("{}: String", p))
        .collect::<Vec<_>>()
        .join(", ");

    if uses_state {
        if params.is_empty() {
            r#"use crate::AppState;
use rejoice::{Req, Res, html};

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    let _ = (state, req); // Silence unused warnings

    res.html(html! {
        h1 { "Hello, world!" }
    })
}
"#
            .to_string()
        } else {
            format!(
                r#"use crate::AppState;
use rejoice::{{Req, Res, html}};

pub async fn get(state: AppState, req: Req, res: Res, {params_typed}) -> Res {{
    let _ = (state, req); // Silence unused warnings

    res.html(html! {{
        h1 {{ "Route params: {params_str}" }}
        p {{ ({params_str}) }}
    }})
}}
"#
            )
        }
    } else if params.is_empty() {
        r#"use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    let _ = req; // Silence unused warning

    res.html(html! {
        h1 { "Hello, world!" }
    })
}
"#
        .to_string()
    } else {
        format!(
            r#"use rejoice::{{Req, Res, html}};

pub async fn get(req: Req, res: Res, {params_typed}) -> Res {{
    let _ = req; // Silence unused warning

    res.html(html! {{
        h1 {{ "Route params: {params_str}" }}
        p {{ ({params_str}) }}
    }})
}}
"#
        )
    }
}

/// Check if a path is in the routes directory
fn is_route_file(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    (path_str.contains("/routes/") || path_str.contains("\\routes\\"))
        && path_str.ends_with(".rs")
        && !path_str.ends_with("mod.rs")
}

/// Generate boilerplate for a newly created route file if it's empty
pub fn maybe_generate_boilerplate(path: &Path) {
    // Only process route files
    if !is_route_file(path) {
        return;
    }

    // Check if file exists and is empty (or very small - just whitespace)
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return, // File might not exist yet or be unreadable
    };

    // Only generate boilerplate for empty files
    if !content.trim().is_empty() {
        return;
    }

    let uses_state = app_uses_state();
    let file_stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let boilerplate = if file_stem == "layout" {
        generate_layout_boilerplate(uses_state)
    } else {
        let params = extract_params_from_path(path);
        generate_route_boilerplate(uses_state, &params)
    };

    if fs::write(path, &boilerplate).is_ok() {
        let relative_path = path
            .strip_prefix(std::env::current_dir().unwrap_or_default())
            .unwrap_or(path);
        println!(
            "  {} {} {}",
            "+".green().bold(),
            "Generated boilerplate for".dimmed(),
            relative_path.display().to_string().cyan()
        );
    }
}
