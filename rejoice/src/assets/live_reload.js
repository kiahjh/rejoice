(function () {
  const ws = new WebSocket("ws://localhost:3001/__reload");

  async function fetchWithRetry(url, maxRetries = 50, delay = 100) {
    for (let i = 0; i < maxRetries; i++) {
      try {
        const response = await fetch(url);
        if (response.ok) return response;
      } catch (e) {}
      await new Promise((r) => setTimeout(r, delay));
    }
    throw new Error("Server not ready");
  }

  ws.onmessage = async function (event) {
    if (event.data === "full") {
      // Full reload needed (client JS changed)
      location.reload();
      return;
    }
    if (event.data === "reload") {
      try {
        const response = await fetchWithRetry(location.href);
        const html = await response.text();
        const parser = new DOMParser();
        const newDoc = parser.parseFromString(html, "text/html");

        // Swap body content
        document.body.innerHTML = newDoc.body.innerHTML;
        
        // Sync body attributes (for classes on <body> in layouts)
        Array.from(newDoc.body.attributes).forEach((attr) => {
          document.body.setAttribute(attr.name, attr.value);
        });
        Array.from(document.body.attributes).forEach((attr) => {
          if (!newDoc.body.hasAttribute(attr.name)) {
            document.body.removeAttribute(attr.name);
          }
        });

        // Sync html element attributes (for classes on <html> in layouts)
        Array.from(newDoc.documentElement.attributes).forEach((attr) => {
          document.documentElement.setAttribute(attr.name, attr.value);
        });
        Array.from(document.documentElement.attributes).forEach((attr) => {
          if (!newDoc.documentElement.hasAttribute(attr.name)) {
            document.documentElement.removeAttribute(attr.name);
          }
        });

        // Update title if changed
        if (newDoc.title !== document.title) {
          document.title = newDoc.title;
        }

        // Refresh stylesheets with cache-busting
        const cacheBuster = Date.now();
        document.querySelectorAll('link[rel="stylesheet"]').forEach((link) => {
          const href = link.getAttribute("href");
          if (href) {
            const url = new URL(href, location.origin);
            url.searchParams.set("_t", cacheBuster);
            link.setAttribute("href", url.toString());
          }
        });

        // Re-hydrate islands after body swap (defer to next frame for DOM to settle)
        requestAnimationFrame(() => {
          if (typeof window.__hydrateIslands === "function") {
            window.__hydrateIslands();
          }
        });
      } catch (e) {
        // Fallback to full reload on error
        location.reload();
      }
    }
  };
  ws.onclose = function () {
    setTimeout(function () {
      location.reload();
    }, 1000);
  };
})();
