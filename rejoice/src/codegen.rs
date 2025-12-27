use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Call this from your build.rs to generate file-based routes.
///
/// # Example
///
/// ```ignore
/// // build.rs
/// fn main() {
///     rejoice::codegen::generate_routes();
/// }
/// ```
pub fn generate_routes() {
    let routes_dir = Path::new("src/routes");
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = Path::new(&out_dir).join("routes_generated.rs");

    // Generate src/routes.rs for rust-analyzer support
    let mut routes_mod = String::new();
    generate_routes_mod(routes_dir, routes_dir, "", &mut routes_mod);
    fs::write("src/routes.rs", &routes_mod).expect("Failed to write src/routes.rs");

    // Collect layouts and routes
    let mut layouts: HashMap<String, String> = HashMap::new(); // dir_path -> mod_name
    let mut routes: Vec<RouteInfo> = Vec::new();

    collect_layouts_and_routes(routes_dir, "", "", &mut layouts, &mut routes);

    // Generate the output
    let mut output = String::new();

    // Use absolute path to src/routes.rs
    let routes_rs_path =
        fs::canonicalize("src/routes.rs").expect("Failed to canonicalize src/routes.rs");
    output.push_str(&format!("#[path = {:?}]\n", routes_rs_path.display()));
    output.push_str("mod routes;\n\n");

    // Generate wrapper handlers for routes with layouts
    for route in &routes {
        if let Some(wrapper) = generate_wrapper_handler(route, &layouts) {
            output.push_str(&wrapper);
            output.push_str("\n\n");
        }
    }

    // Generate router
    output.push_str("pub fn create_router() -> axum::Router {\n");
    output.push_str("    axum::Router::new()\n");

    for route in &routes {
        let handler = if has_layouts(route, &layouts) {
            format!("wrapper_{}", route.mod_name)
        } else {
            format!("routes::{}::page", route.mod_name)
        };
        output.push_str(&format!(
            "        .route({:?}, axum::routing::get({}))\n",
            route.url_path, handler
        ));
    }

    output.push_str("}\n");

    fs::write(&out_path, &output).expect("Failed to write routes_generated.rs");

    // Tell Cargo to rerun if routes change
    println!("cargo:rerun-if-changed=src/routes");
}

struct RouteInfo {
    url_path: String,
    mod_name: String,
    /// Directory path relative to routes dir (e.g., "" for root, "users" for users/)
    dir_path: String,
    /// Parameter name if this is a dynamic route (e.g., "id" for [id].rs)
    param: Option<String>,
}

fn collect_layouts_and_routes(
    dir: &Path,
    url_prefix: &str,
    mod_prefix: &str,
    layouts: &mut HashMap<String, String>,
    routes: &mut Vec<RouteInfo>,
) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.path());

    // Current directory path for layout lookup
    let dir_path = mod_prefix.replace('_', "/");

    for entry in entries {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() {
            let new_url_prefix = format!("{}/{}", url_prefix, file_name);
            let new_mod_prefix = if mod_prefix.is_empty() {
                file_name.to_string()
            } else {
                format!("{}_{}", mod_prefix, file_name)
            };
            collect_layouts_and_routes(&path, &new_url_prefix, &new_mod_prefix, layouts, routes);
        } else if file_name.ends_with(".rs") && file_name != "mod.rs" {
            let stem = path.file_stem().unwrap().to_str().unwrap();

            // Handle layout.rs
            if stem == "layout" {
                let layout_mod_name = if mod_prefix.is_empty() {
                    "layout".to_string()
                } else {
                    format!("{}_layout", mod_prefix)
                };
                layouts.insert(dir_path.clone(), layout_mod_name);
                continue;
            }

            let (file_mod_name, route_segment, param) = if stem.starts_with('[') && stem.ends_with(']') {
                let param_name = &stem[1..stem.len() - 1];
                (format!("param_{}", param_name), format!("{{{}}}", param_name), Some(param_name.to_string()))
            } else {
                (stem.to_string(), stem.to_string(), None)
            };

            let url_path = if stem == "index" {
                if url_prefix.is_empty() {
                    "/".to_string()
                } else {
                    url_prefix.to_string()
                }
            } else {
                format!("{}/{}", url_prefix, route_segment)
            };

            let full_mod_name = if mod_prefix.is_empty() {
                file_mod_name
            } else {
                format!("{}_{}", mod_prefix, file_mod_name)
            };

            routes.push(RouteInfo {
                url_path,
                mod_name: full_mod_name,
                dir_path: dir_path.clone(),
                param,
            });
        }
    }
}

fn has_layouts(route: &RouteInfo, layouts: &HashMap<String, String>) -> bool {
    get_layout_chain(route, layouts).is_some()
}

/// Returns the chain of layout module names from outermost to innermost
fn get_layout_chain(route: &RouteInfo, layouts: &HashMap<String, String>) -> Option<Vec<String>> {
    let mut chain = Vec::new();

    // Check root layout
    if let Some(layout_mod) = layouts.get("") {
        chain.push(layout_mod.clone());
    }

    // Check each directory level
    if !route.dir_path.is_empty() {
        let parts: Vec<&str> = route.dir_path.split('/').collect();
        let mut current_path = String::new();

        for part in parts {
            if !current_path.is_empty() {
                current_path.push('/');
            }
            current_path.push_str(part);

            if let Some(layout_mod) = layouts.get(&current_path) {
                chain.push(layout_mod.clone());
            }
        }
    }

    if chain.is_empty() {
        None
    } else {
        Some(chain)
    }
}

fn generate_wrapper_handler(route: &RouteInfo, layouts: &HashMap<String, String>) -> Option<String> {
    let chain = get_layout_chain(route, layouts)?;

    let mut output = String::new();

    // Generate the wrapper function with appropriate signature
    if let Some(param) = &route.param {
        output.push_str(&format!(
            "async fn wrapper_{}(axum::extract::Path({param}): axum::extract::Path<String>) -> maud::Markup {{\n",
            route.mod_name
        ));
        output.push_str(&format!(
            "    let content = routes::{}::page(axum::extract::Path({param})).await;\n",
            route.mod_name
        ));
    } else {
        output.push_str(&format!(
            "async fn wrapper_{}() -> maud::Markup {{\n",
            route.mod_name
        ));
        output.push_str(&format!(
            "    let content = routes::{}::page().await;\n",
            route.mod_name
        ));
    }

    // Wrap with each layout from innermost to outermost
    for layout_mod in chain.iter().rev() {
        output.push_str(&format!(
            "    let content = routes::{}::layout(content).await;\n",
            layout_mod
        ));
    }

    output.push_str("    content\n");
    output.push_str("}");

    Some(output)
}

fn generate_routes_mod(base_dir: &Path, dir: &Path, mod_prefix: &str, output: &mut String) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() {
            let new_prefix = if mod_prefix.is_empty() {
                file_name.to_string()
            } else {
                format!("{}_{}", mod_prefix, file_name)
            };
            generate_routes_mod(base_dir, &path, &new_prefix, output);
        } else if file_name.ends_with(".rs") && file_name != "mod.rs" {
            let stem = path.file_stem().unwrap().to_str().unwrap();

            let file_mod_name = if stem.starts_with('[') && stem.ends_with(']') {
                let param = &stem[1..stem.len() - 1];
                format!("param_{}", param)
            } else {
                stem.to_string()
            };

            let full_mod_name = if mod_prefix.is_empty() {
                file_mod_name
            } else {
                format!("{}_{}", mod_prefix, file_mod_name)
            };

            let rel_path = path.strip_prefix(base_dir).unwrap();
            output.push_str(&format!("#[path = \"routes/{}\"]\n", rel_path.display()));
            output.push_str(&format!("pub mod {};\n", full_mod_name));
        }
    }
}
