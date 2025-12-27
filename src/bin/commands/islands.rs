use std::path::Path;

/// Generates the islands.tsx registry file from all component files in client/
pub fn generate_islands_registry() {
    let client_dir = Path::new("client");
    let Ok(entries) = std::fs::read_dir(client_dir) else {
        return;
    };

    let mut components: Vec<String> = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(ext) = path.extension()
            && (ext == "tsx" || ext == "jsx")
            && let Some(stem) = path.file_stem()
        {
            let name = stem.to_string_lossy().to_string();
            // Skip the islands.tsx file itself
            if name != "islands" {
                components.push(name);
            }
        }
    }

    if components.is_empty() {
        return;
    }

    let output = generate_islands_code(&components);
    std::fs::write(client_dir.join("islands.tsx"), output).expect("Failed to write islands.tsx");
}

fn generate_islands_code(components: &[String]) -> String {
    let mut output = String::new();

    // Imports
    output.push_str("import { render } from \"solid-js/web\";\n\n");

    for name in components {
        output.push_str(&format!("import {name} from \"./{name}\";\n"));
    }

    // Islands registry
    output.push_str("\nconst islands: Record<string, any> = {\n");
    for name in components {
        output.push_str(&format!("  {name},\n"));
    }
    output.push_str("};\n\n");

    // Hydration function
    output.push_str(HYDRATE_ISLANDS_CODE);

    output
}

const HYDRATE_ISLANDS_CODE: &str = r#"function hydrateIslands() {
  document.querySelectorAll("[data-island]").forEach((el) => {
    const name = el.getAttribute("data-island");
    const props = JSON.parse(el.getAttribute("data-props") || "{}");
    const Component = islands[name!];
    if (Component) {
      el.innerHTML = "";
      render(() => <Component {...props} />, el);
    }
  });
}

// Expose for live reload re-hydration
(window as any).__hydrateIslands = hydrateIslands;

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", hydrateIslands);
} else {
  hydrateIslands();
}
"#;
