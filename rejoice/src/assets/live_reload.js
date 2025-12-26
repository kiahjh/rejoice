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
