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

    let mut output = String::new();
    let mut route_registrations = Vec::new();

    output.push_str("mod routes {\n");
    scan_routes(routes_dir, "", &mut output, &mut route_registrations, 1);
    output.push_str("}\n\n");

    output.push_str("pub fn create_router() -> axum::Router {\n");
    output.push_str("    axum::Router::new()\n");
    for route in &route_registrations {
        output.push_str(route);
        output.push('\n');
    }
    output.push_str("}\n");

    fs::write(&out_path, &output).expect("Failed to write routes_generated.rs");

    // Tell Cargo to rerun if routes change
    println!("cargo:rerun-if-changed=src/routes");
}

fn scan_routes(
    dir: &Path,
    url_prefix: &str,
    output: &mut String,
    routes: &mut Vec<String>,
    depth: usize,
) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let indent = "    ".repeat(depth);
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() {
            // Create a submodule for the directory
            let mod_name = file_name.to_string();
            let new_url_prefix = format!("{}/{}", url_prefix, file_name);

            output.push_str(&format!("{}pub mod {} {{\n", indent, mod_name));
            scan_routes(&path, &new_url_prefix, output, routes, depth + 1);
            output.push_str(&format!("{}}}\n", indent));
        } else if file_name.ends_with(".rs") && file_name != "mod.rs" {
            let stem = path.file_stem().unwrap().to_str().unwrap();

            // Convert [param] to {param} for axum, and use valid Rust ident for mod name
            let (mod_name, route_segment) = if stem.starts_with('[') && stem.ends_with(']') {
                let param = &stem[1..stem.len() - 1];
                (format!("param_{}", param), format!("{{{}}}", param))
            } else {
                (stem.to_string(), stem.to_string())
            };

            // Build the URL path
            let url_path = if stem == "index" {
                if url_prefix.is_empty() {
                    "/".to_string()
                } else {
                    url_prefix.to_string()
                }
            } else {
                format!("{}/{}", url_prefix, route_segment)
            };

            // Build the module path for the handler
            let mod_path = build_mod_path(url_prefix, &mod_name);

            // Add the module with #[path] to point to the actual file
            output.push_str(&format!(
                "{}#[path = {:?}]\n",
                indent,
                path.canonicalize().unwrap().display()
            ));
            output.push_str(&format!("{}pub mod {};\n", indent, mod_name));

            routes.push(format!(
                "        .route({:?}, axum::routing::get(routes::{}::handler))",
                url_path, mod_path
            ));
        }
    }
}

fn build_mod_path(url_prefix: &str, mod_name: &str) -> String {
    if url_prefix.is_empty() {
        mod_name.to_string()
    } else {
        let parts: Vec<&str> = url_prefix
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        if parts.is_empty() {
            mod_name.to_string()
        } else {
            format!("{}::{}", parts.join("::"), mod_name)
        }
    }
}
