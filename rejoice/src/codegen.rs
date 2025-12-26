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

    // Generate src/routes.rs for rust-analyzer support
    let mut routes_mod = String::new();
    generate_routes_mod(routes_dir, routes_dir, "", &mut routes_mod);
    fs::write("src/routes.rs", &routes_mod).expect("Failed to write src/routes.rs");

    // Use absolute path to src/routes.rs
    let routes_rs_path =
        fs::canonicalize("src/routes.rs").expect("Failed to canonicalize src/routes.rs");
    output.push_str(&format!("#[path = {:?}]\n", routes_rs_path.display()));
    output.push_str("mod routes;\n\n");

    // Collect route registrations
    collect_routes(routes_dir, "", "", &mut route_registrations);

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

fn collect_routes(dir: &Path, url_prefix: &str, mod_prefix: &str, routes: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.path());

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
            collect_routes(&path, &new_url_prefix, &new_mod_prefix, routes);
        } else if file_name.ends_with(".rs") && file_name != "mod.rs" {
            let stem = path.file_stem().unwrap().to_str().unwrap();

            let (file_mod_name, route_segment) = if stem.starts_with('[') && stem.ends_with(']') {
                let param = &stem[1..stem.len() - 1];
                (format!("param_{}", param), format!("{{{}}}", param))
            } else {
                (stem.to_string(), stem.to_string())
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

            routes.push(format!(
                "        .route({:?}, axum::routing::get(routes::{}::handler))",
                url_path, full_mod_name
            ));
        }
    }
}
