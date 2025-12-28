use std::path::Path;

/// Checks if there are any island components (tsx/jsx files) in client/
pub fn has_island_components() -> bool {
    let client_dir = Path::new("client");
    let Ok(entries) = std::fs::read_dir(client_dir) else {
        return false;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(ext) = path.extension()
            && (ext == "tsx" || ext == "jsx")
            && let Some(stem) = path.file_stem()
        {
            let name = stem.to_string_lossy().to_string();
            if name != "islands" && name != "styles" {
                return true;
            }
        }
    }

    false
}

/// Generates the islands.tsx registry file from all component files in client/
/// Returns true if islands were found and the file was generated.
pub fn generate_islands_registry() -> bool {
    let client_dir = Path::new("client");
    let Ok(entries) = std::fs::read_dir(client_dir) else {
        return false;
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
        return false;
    }

    let output = generate_islands_code(&components);
    std::fs::write(client_dir.join("islands.tsx"), output).expect("Failed to write islands.tsx");
    true
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

/// Generates a vite.config.ts appropriate for the current project
/// If has_islands is true, includes islands.tsx as an input
pub fn generate_vite_config(has_islands: bool) {
    let content = if has_islands {
        r#"import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [solid(), tailwindcss()],
  build: {
    outDir: "dist",
    rollupOptions: {
      input: {
        islands: "client/islands.tsx",
        styles: "client/styles.css",
      },
      output: {
        entryFileNames: "[name].js",
        assetFileNames: "[name].[ext]",
      },
    },
  },
});
"#
    } else {
        r#"import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [tailwindcss()],
  build: {
    outDir: "dist",
    rollupOptions: {
      input: {
        styles: "client/styles.css",
      },
      output: {
        assetFileNames: "[name].[ext]",
      },
    },
  },
});
"#
    };

    std::fs::write("vite.config.ts", content).expect("Failed to write vite.config.ts");
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
