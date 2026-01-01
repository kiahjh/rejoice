import { onMount } from "solid-js";
import { codeToHtml } from "shiki";

// This component handles code highlighting and copy functionality
// It runs once on mount and sets up all code blocks on the page

export default function CodeHighlighter() {
  onMount(async () => {
    await highlightAllCodeBlocks();
    setupCopyButtons();

    // Expose functions for re-initialization after HMR
    (window as any).__highlightCode = highlightAllCodeBlocks;
    (window as any).__copyCode = copyCode;
  });

  return null;
}

async function highlightAllCodeBlocks() {
  const codeBlocks = document.querySelectorAll(".code-block[data-code]");

  for (const block of codeBlocks) {
    const code = block.getAttribute("data-code") || "";
    const lang = block.getAttribute("data-lang") || "text";

    // Skip if already highlighted
    if (block.classList.contains("highlighted")) continue;

    try {
      const html = await codeToHtml(code, {
        lang: mapLanguage(lang),
        theme: "vitesse-dark",
      });

      // Insert the highlighted code
      const wrapper = document.createElement("div");
      wrapper.innerHTML = html;

      const shikiPre = wrapper.querySelector("pre");
      if (shikiPre) {
        const existingPre = block.querySelector("pre");
        if (existingPre) {
          existingPre.replaceWith(shikiPre);
        }
      }

      block.classList.add("highlighted");
    } catch (e) {
      // Fallback to text if language isn't supported
      console.warn(
        `Shiki: Could not highlight language "${lang}", falling back to text`
      );
      try {
        const html = await codeToHtml(code, {
          lang: "text",
          theme: "vitesse-dark",
        });
        const wrapper = document.createElement("div");
        wrapper.innerHTML = html;
        const shikiPre = wrapper.querySelector("pre");
        if (shikiPre) {
          const existingPre = block.querySelector("pre");
          if (existingPre) {
            existingPre.replaceWith(shikiPre);
          }
        }
        block.classList.add("highlighted");
      } catch (e2) {
        console.error("Shiki highlighting failed:", e2);
      }
    }
  }
}

function setupCopyButtons() {
  // The copy function is exposed globally and called via onclick
  // This allows it to work with server-rendered buttons
}

function copyCode(button: HTMLButtonElement) {
  const codeBlock = button.closest(".code-block");
  if (!codeBlock) return;

  const code = codeBlock.getAttribute("data-code") || "";

  navigator.clipboard.writeText(code).then(() => {
    // Show success state
    button.classList.add("copied");
    const copyIcon = button.querySelector(".copy-icon");
    const checkIcon = button.querySelector(".check-icon");

    if (copyIcon) copyIcon.classList.add("hidden");
    if (checkIcon) checkIcon.classList.remove("hidden");

    // Reset after 2 seconds
    setTimeout(() => {
      button.classList.remove("copied");
      if (copyIcon) copyIcon.classList.remove("hidden");
      if (checkIcon) checkIcon.classList.add("hidden");
    }, 2000);
  });
}

// Map common language aliases
function mapLanguage(lang: string): string {
  const aliases: Record<string, string> = {
    rs: "rust",
    js: "javascript",
    ts: "typescript",
    tsx: "tsx",
    jsx: "jsx",
    sh: "bash",
    shell: "bash",
    yml: "yaml",
    md: "markdown",
    toml: "toml",
    json: "json",
    html: "html",
    css: "css",
  };
  return aliases[lang] || lang;
}
