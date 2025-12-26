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
