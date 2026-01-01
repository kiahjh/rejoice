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

    // Generate src/routes.rs for rust-analyzer support
    let mut routes_mod = String::new();
    generate_routes_mod(routes_dir, routes_dir, "", &mut routes_mod);
    fs::write("src/routes.rs", &routes_mod).expect("Failed to write src/routes.rs");

    // Collect layouts and routes
    let mut layouts: HashMap<String, String> = HashMap::new();
    let mut routes: Vec<RouteInfo> = Vec::new();

    collect_layouts_and_routes(routes_dir, "", "", &mut layouts, &mut routes);

    // Use absolute path to src/routes.rs
    let routes_rs_path =
        fs::canonicalize("src/routes.rs").expect("Failed to canonicalize src/routes.rs");

    // Generate stateless version
    let stateless_output = generate_routes_file(&routes_rs_path, &routes, &layouts, true);
    let stateless_path = Path::new(&out_dir).join("routes_generated_stateless.rs");
    fs::write(&stateless_path, &stateless_output)
        .expect("Failed to write routes_generated_stateless.rs");

    // Generate stateful version
    let stateful_output = generate_routes_file(&routes_rs_path, &routes, &layouts, false);
    let stateful_path = Path::new(&out_dir).join("routes_generated_stateful.rs");
    fs::write(&stateful_path, &stateful_output)
        .expect("Failed to write routes_generated_stateful.rs");

    // Tell Cargo to rerun if routes change
    println!("cargo:rerun-if-changed=src/routes");
}

fn generate_routes_file(
    routes_rs_path: &Path,
    routes: &[RouteInfo],
    layouts: &HashMap<String, String>,
    stateless: bool,
) -> String {
    let mut output = String::new();

    output.push_str(&format!("#[path = {:?}]\n", routes_rs_path.display()));
    output.push_str("mod routes;\n\n");

    // Generate wrapper handlers for routes with layouts
    for route in routes {
        if let Some(wrapper) = generate_wrapper_handler(route, layouts, stateless) {
            output.push_str(&wrapper);
            output.push_str("\n\n");
        }
    }

    // Generate router
    output.push_str("pub fn create_router() -> rejoice::Router<__RejoiceState> {\n");
    output.push_str("    rejoice::Router::new()\n");

    for route in routes {
        let handler = if has_layouts(route, layouts) {
            format!("wrapper_{}", route.mod_name)
        } else {
            format!("routes::{}::page", route.mod_name)
        };
        output.push_str(&format!(
            "        .route({:?}, rejoice::routing::get({}))\n",
            route.url_path, handler
        ));
    }

    output.push_str("}\n");

    output
}

struct RouteInfo {
    url_path: String,
    mod_name: String,
    dir_path: String,
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

    let dir_path = mod_prefix.replace('_', "/");

    for entry in entries {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() {
            // Convert underscores to hyphens for directory URL segments
            let url_segment = file_name.replace('_', "-");
            let new_url_prefix = format!("{}/{}", url_prefix, url_segment);
            let new_mod_prefix = if mod_prefix.is_empty() {
                file_name.to_string()
            } else {
                format!("{}_{}", mod_prefix, file_name)
            };
            collect_layouts_and_routes(&path, &new_url_prefix, &new_mod_prefix, layouts, routes);
        } else if file_name.ends_with(".rs") && file_name != "mod.rs" {
            let stem = path.file_stem().unwrap().to_str().unwrap();

            if stem == "layout" {
                let layout_mod_name = if mod_prefix.is_empty() {
                    "layout".to_string()
                } else {
                    format!("{}_layout", mod_prefix)
                };
                layouts.insert(dir_path.clone(), layout_mod_name);
                continue;
            }

            let (file_mod_name, route_segment, param) =
                if stem.starts_with('[') && stem.ends_with(']') {
                    let param_name = &stem[1..stem.len() - 1];
                    (
                        format!("param_{}", param_name),
                        format!("{{{}}}", param_name),
                        Some(param_name.to_string()),
                    )
                } else {
                    // Convert underscores to hyphens for URL segments
                    // e.g., project_structure.rs -> /project-structure
                    let url_segment = stem.replace('_', "-");
                    (stem.to_string(), url_segment, None)
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

fn get_layout_chain(route: &RouteInfo, layouts: &HashMap<String, String>) -> Option<Vec<String>> {
    let mut chain = Vec::new();

    if let Some(layout_mod) = layouts.get("") {
        chain.push(layout_mod.clone());
    }

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

fn generate_wrapper_handler(
    route: &RouteInfo,
    layouts: &HashMap<String, String>,
    stateless: bool,
) -> Option<String> {
    let chain = get_layout_chain(route, layouts)?;

    let mut output = String::new();

    // Generate the wrapper function signature
    if let Some(param) = &route.param {
        output.push_str(&format!(
            "async fn wrapper_{}(\n    rejoice::State(state): rejoice::State<__RejoiceState>,\n    req: rejoice::Req,\n    res: rejoice::Res,\n    rejoice::Path({param}): rejoice::Path<String>,\n) -> rejoice::Res {{\n",
            route.mod_name
        ));

        // Call page with appropriate args
        if stateless {
            output.push_str(&format!(
                "    let _ = state;\n    let res = routes::{}::page(req.clone(), res, {param}).await;\n",
                route.mod_name
            ));
        } else {
            output.push_str(&format!(
                "    let res = routes::{}::page(state.clone(), req.clone(), res, {param}).await;\n",
                route.mod_name
            ));
        }
    } else {
        output.push_str(&format!(
            "async fn wrapper_{}(\n    rejoice::State(state): rejoice::State<__RejoiceState>,\n    req: rejoice::Req,\n    res: rejoice::Res,\n) -> rejoice::Res {{\n",
            route.mod_name
        ));

        // Call page with appropriate args
        if stateless {
            output.push_str(&format!(
                "    let _ = state;\n    let res = routes::{}::page(req.clone(), res).await;\n",
                route.mod_name
            ));
        } else {
            output.push_str(&format!(
                "    let res = routes::{}::page(state.clone(), req.clone(), res).await;\n",
                route.mod_name
            ));
        }
    }

    // Only wrap with layouts if the response is HTML
    output.push_str("    if !res.is_html() {\n");
    output.push_str("        return res;\n");
    output.push_str("    }\n");

    // Extract HTML content, wrap with layouts
    output.push_str("    let html_content = res.take_html().unwrap();\n");
    output.push_str("    let children: rejoice::Children = rejoice::PreEscaped(html_content);\n");

    // Wrap with each layout from innermost to outermost
    for (i, layout_mod) in chain.iter().rev().enumerate() {
        let children_var = if i == 0 {
            "children"
        } else {
            &format!("children_{}", i)
        };
        let next_children_var = format!("children_{}", i + 1);

        if stateless {
            output.push_str(&format!(
                "    let layout_res = routes::{}::layout(req.clone(), rejoice::Res::new(), {}).await;\n",
                layout_mod, children_var
            ));
        } else {
            output.push_str(&format!(
                "    let layout_res = routes::{}::layout(state.clone(), req.clone(), rejoice::Res::new(), {}).await;\n",
                layout_mod, children_var
            ));
        }

        if i < chain.len() - 1 {
            output.push_str("    if !layout_res.is_html() {\n");
            output.push_str("        return layout_res;\n");
            output.push_str("    }\n");
            output.push_str(&format!(
                "    let {}: rejoice::Children = rejoice::PreEscaped(layout_res.take_html().unwrap());\n",
                next_children_var
            ));
        } else {
            output.push_str("    if !layout_res.is_html() {\n");
            output.push_str("        return layout_res;\n");
            output.push_str("    }\n");
            output.push_str("    layout_res\n");
        }
    }

    output.push('}');

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
