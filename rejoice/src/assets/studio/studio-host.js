/**
 * Rejoice Studio
 * 
 * A delightful visual development environment.
 * Playful but productive. Fun but functional.
 */

// =============================================================================
// State
// =============================================================================

const State = {
  panelOpen: false,
  selectMode: false,
  selectedElement: null,
  components: [],
  ws: null,
  wsConnected: false,
  activeTab: "inspect",
  iframe: null,
  panelWidth: 380,
  isResizing: false,
};

const MIN_PANEL_WIDTH = 380;

// =============================================================================
// Init
// =============================================================================

function init() {
  document.head.insertAdjacentHTML('beforeend', `
    <link href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  `);
  
  document.body.innerHTML = `
    <div id="studio">
      <div id="stage">
        <div id="canvas">
          <iframe id="iframe" src="${getAppUrl()}"></iframe>
        </div>
      </div>
      
      <div id="highlight">
        <div class="highlight-label"></div>
      </div>
      
      <div id="resize-handle"></div>
      
      <aside id="panel">
        <header id="header">
          <div id="brand">
            <div class="brand-icon">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
                <path d="M12 2L2 7l10 5 10-5-10-5z" fill="url(#g1)"/>
                <path d="M2 17l10 5 10-5" stroke="url(#g1)" stroke-width="2.5" stroke-linecap="round" opacity="0.5"/>
                <path d="M2 12l10 5 10-5" stroke="url(#g1)" stroke-width="2.5" stroke-linecap="round" opacity="0.8"/>
                <defs>
                  <linearGradient id="g1" x1="2" y1="2" x2="22" y2="22">
                    <stop stop-color="#f0abfc"/>
                    <stop offset="1" stop-color="#818cf8"/>
                  </linearGradient>
                </defs>
              </svg>
            </div>
            <span>Studio</span>
          </div>
          
          <div id="tools">
            <button id="select-btn" class="tool-btn" title="Select (S)">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z"/>
              </svg>
              <span class="tool-label">Select</span>
            </button>
          </div>
          
          <button id="close-btn" title="Close (Esc)">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
              <path d="M18 6L6 18M6 6l12 12"/>
            </svg>
          </button>
        </header>
        
        <nav id="tabs">
          <button class="tab active" data-tab="inspect">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="11" cy="11" r="8"/>
              <path d="m21 21-4.35-4.35"/>
            </svg>
            Inspect
          </button>
          <button class="tab" data-tab="elements">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9V3m0 9a9 9 0 00-9 9"/>
            </svg>
            Elements
          </button>
          <button class="tab" data-tab="components">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="3" y="3" width="7" height="7" rx="1"/>
              <rect x="14" y="3" width="7" height="7" rx="1"/>
              <rect x="3" y="14" width="7" height="7" rx="1"/>
              <rect x="14" y="14" width="7" height="7" rx="1"/>
            </svg>
            Components
          </button>
        </nav>
        
        <main id="body">
          <section class="tab-panel active" data-tab="inspect">
            <div id="inspect-empty">
              <div class="empty-visual">
                <div class="cursor-icon">
                  <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z"/>
                  </svg>
                </div>
                <div class="sparkle s1">✦</div>
                <div class="sparkle s2">✦</div>
                <div class="sparkle s3">·</div>
              </div>
              <h3>Pick something!</h3>
              <p>Press <kbd>S</kbd> and click any element to start inspecting</p>
            </div>
            <div id="inspect-content"></div>
          </section>
          
          <section class="tab-panel" data-tab="elements">
            <div id="tree"></div>
          </section>
          
          <section class="tab-panel" data-tab="components">
            <div id="components"></div>
          </section>
        </main>
        
        <footer id="footer">
          <div class="shortcut-hint">
            <kbd>⌘</kbd><kbd>.</kbd> toggle · <kbd>S</kbd> select · <kbd>esc</kbd> close
          </div>
        </footer>
      </aside>
      
      <button id="toggle" title="Open Studio (⌘.)">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
          <path d="M12 2L2 7l10 5 10-5-10-5z" fill="url(#g2)"/>
          <path d="M2 17l10 5 10-5" stroke="url(#g2)" stroke-width="2" stroke-linecap="round" opacity="0.5"/>
          <path d="M2 12l10 5 10-5" stroke="url(#g2)" stroke-width="2" stroke-linecap="round" opacity="0.8"/>
          <defs>
            <linearGradient id="g2" x1="2" y1="2" x2="22" y2="22">
              <stop stop-color="#f0abfc"/>
              <stop offset="1" stop-color="#818cf8"/>
            </linearGradient>
          </defs>
        </svg>
      </button>
    </div>
  `;
  
  injectStyles();
  
  State.iframe = document.getElementById("iframe");
  document.documentElement.style.setProperty("--panel-width", State.panelWidth + "px");
  
  bindEvents();
  connectWS();
  fetchComponents();
}

function getAppUrl() {
  const params = new URLSearchParams(window.location.search);
  const path = params.get('path') || '/';
  return path + (path.includes('?') ? '&' : '?') + '__studio_bridge=1';
}

// =============================================================================
// Events
// =============================================================================

function bindEvents() {
  $("#toggle").addEventListener("click", toggle);
  $("#close-btn").addEventListener("click", toggle);
  $("#select-btn").addEventListener("click", toggleSelect);
  
  $$(".tab").forEach(t => t.addEventListener("click", () => switchTab(t.dataset.tab)));
  
  document.addEventListener("keydown", e => {
    if ((e.metaKey || e.ctrlKey) && e.key === ".") {
      e.preventDefault();
      toggle();
    }
    if (e.key === "Escape") {
      State.selectMode ? toggleSelect() : State.panelOpen && toggle();
    }
    if (e.key === "s" && State.panelOpen && !e.target.matches("input,textarea")) {
      e.preventDefault();
      toggleSelect();
    }
  });
  
  window.addEventListener("message", onMessage);
  State.iframe.addEventListener("load", syncUrl);
  
  // Resize
  const handle = $("#resize-handle");
  handle.addEventListener("mousedown", e => {
    e.preventDefault();
    State.isResizing = true;
    document.body.classList.add("resizing");
  });
  document.addEventListener("mousemove", e => {
    if (!State.isResizing) return;
    const w = Math.max(MIN_PANEL_WIDTH, window.innerWidth - e.clientX);
    State.panelWidth = w;
    document.documentElement.style.setProperty("--panel-width", w + "px");
  });
  document.addEventListener("mouseup", () => {
    if (State.isResizing) {
      State.isResizing = false;
      document.body.classList.remove("resizing");
    }
  });
}

function syncUrl() {
  try {
    const url = new URL(State.iframe.contentWindow.location.href);
    url.searchParams.delete("__studio_bridge");
    const path = url.pathname + url.search;
    history.replaceState(null, "", "/__studio" + (path !== "/" ? "?path=" + encodeURIComponent(path) : ""));
  } catch (e) {}
}

// =============================================================================
// Panel
// =============================================================================

function toggle() {
  State.panelOpen = !State.panelOpen;
  $("#studio").classList.toggle("open", State.panelOpen);
  if (!State.panelOpen && State.selectMode) toggleSelect();
}

function toggleSelect() {
  State.selectMode = !State.selectMode;
  $("#select-btn").classList.toggle("active", State.selectMode);
  $("#studio").classList.toggle("selecting", State.selectMode);
  send({ type: "set-select-mode", enabled: State.selectMode });
  if (!State.selectMode) $("#highlight").style.display = "none";
}

function switchTab(tab) {
  State.activeTab = tab;
  $$(".tab").forEach(t => t.classList.toggle("active", t.dataset.tab === tab));
  $$(".tab-panel").forEach(p => p.classList.toggle("active", p.dataset.tab === tab));
  if (tab === "elements") send({ type: "get-tree" });
}

// =============================================================================
// Iframe Communication
// =============================================================================

function send(msg) {
  State.iframe?.contentWindow?.postMessage(msg, "*");
}

function onMessage(e) {
  const m = e.data;
  if (!m || typeof m !== "object") return;
  
  switch (m.type) {
    case "hover": showHighlight(m.rect, m.tagName); break;
    case "hover-end": $("#highlight").style.display = "none"; break;
    case "selected": onSelect(m); break;
    case "tree-data": renderTree(m.tree); break;
    case "navigate": history.replaceState(null, "", "/__studio?path=" + encodeURIComponent(m.path)); break;
    case "shortcut": handleShortcut(m.key); break;
  }
}

function handleShortcut(key) {
  if (key === "toggle") toggle();
  else if (key === "escape") State.selectMode ? toggleSelect() : State.panelOpen && toggle();
  else if (key === "select" && State.panelOpen) toggleSelect();
}

function showHighlight(r, tagName) {
  const h = $("#highlight");
  const c = $("#canvas").getBoundingClientRect();
  Object.assign(h.style, {
    display: "block",
    left: c.left + r.left + "px",
    top: c.top + r.top + "px",
    width: r.width + "px",
    height: r.height + "px",
  });
  if (tagName) {
    h.querySelector(".highlight-label").textContent = tagName;
  }
}

// =============================================================================
// Inspect
// =============================================================================

function onSelect(m) {
  State.selectedElement = {
    tagName: m.tagName,
    classes: m.classes,
    id: m.id,
    componentName: m.componentName,
    sourceLocation: m.sourceLocation,
    path: m.path,
  };
  
  if (State.selectMode) toggleSelect();
  renderInspect();
  if (State.activeTab !== "inspect") switchTab("inspect");
}

function renderInspect() {
  const el = State.selectedElement;
  const empty = $("#inspect-empty");
  const content = $("#inspect-content");
  
  if (!el) {
    empty.style.display = "";
    content.style.display = "none";
    content.innerHTML = "";
    return;
  }
  
  empty.style.display = "none";
  content.style.display = "block";
  
  content.innerHTML = `
    <div class="selected-card">
      <div class="selected-header">
        <span class="tag-badge">&lt;${el.tagName}&gt;</span>
        ${el.id ? `<span class="id-badge">#${el.id}</span>` : ''}
      </div>
      ${el.componentName ? `
        <div class="component-pill">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="7" height="7" rx="1"/>
            <rect x="14" y="3" width="7" height="7" rx="1"/>
            <rect x="3" y="14" width="7" height="7" rx="1"/>
            <rect x="14" y="14" width="7" height="7" rx="1"/>
          </svg>
          ${el.componentName}
        </div>
      ` : ''}
      ${el.sourceLocation ? `
        <div class="source-line">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M14.5 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V7.5L14.5 2z"/>
            <polyline points="14,2 14,8 20,8"/>
          </svg>
          ${el.sourceLocation}
        </div>
      ` : ''}
    </div>
    
    <div class="section">
      <label class="section-label">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M20.59 13.41l-7.17 7.17a2 2 0 01-2.83 0L2 12V2h10l8.59 8.59a2 2 0 010 2.82z"/>
          <line x1="7" y1="7" x2="7.01" y2="7"/>
        </svg>
        Classes
      </label>
      <textarea id="classes-input" spellcheck="false" placeholder="flex items-center gap-4 ...">${el.classes || ''}</textarea>
      <button id="apply-btn">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="20,6 9,17 4,12"/>
        </svg>
        Apply Changes
      </button>
    </div>
  `;
  
  $("#apply-btn").addEventListener("click", () => applyClasses($("#classes-input").value));
  $("#classes-input").addEventListener("keydown", e => {
    if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
      e.preventDefault();
      applyClasses(e.target.value);
    }
  });
}

function applyClasses(classes) {
  const el = State.selectedElement;
  if (!el || el.classes === classes) return;
  
  const old = el.classes || "";
  send({ type: "apply-classes", path: el.path, classes });
  el.classes = classes;
  
  if (el.sourceLocation) {
    const [file, line] = el.sourceLocation.split(":");
    if (file && line) {
      sendWS({
        type: "edit_file",
        file,
        edits: [{ line: parseInt(line), old_text: `class="${old}"`, new_text: `class="${classes}"` }],
      });
    }
  }
  
  toast("✓ Applied!");
}

// =============================================================================
// Tree
// =============================================================================

function renderTree(tree) {
  const el = $("#tree");
  if (!tree) { el.innerHTML = `<p class="empty-msg">Loading...</p>`; return; }
  
  el.innerHTML = renderNode(tree, 0);
  
  el.querySelectorAll(".tree-node").forEach(n => {
    n.addEventListener("click", e => {
      e.stopPropagation();
      el.querySelectorAll(".tree-node").forEach(x => x.classList.remove("selected"));
      n.classList.add("selected");
      send({ type: "select-by-path", path: n.dataset.path });
    });
  });
}

function renderNode(n, depth) {
  if (!n) return "";
  const badge = n.componentName ? `<span class="tree-comp">${n.componentName}</span>` : "";
  let html = `<div class="tree-node" style="--d:${depth}" data-path="${n.path}">
    <span class="tree-tag">${n.tagName}</span>${badge}
  </div>`;
  if (n.children) n.children.forEach(c => html += renderNode(c, depth + 1));
  return html;
}

// =============================================================================
// Components
// =============================================================================

async function fetchComponents() {
  try {
    const r = await fetch("/__studio/registry");
    State.components = (await r.json()).components || [];
    renderComponents();
  } catch (e) {}
}

function renderComponents() {
  const el = $("#components");
  const list = State.components;
  
  if (!list.length) {
    el.innerHTML = `
      <div class="empty-components">
        <div class="empty-visual">
          <div class="cursor-icon">
            <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="3" y="3" width="7" height="7" rx="1"/>
              <rect x="14" y="3" width="7" height="7" rx="1"/>
              <rect x="3" y="14" width="7" height="7" rx="1"/>
              <rect x="14" y="14" width="7" height="7" rx="1"/>
            </svg>
          </div>
          <div class="sparkle s1">✦</div>
          <div class="sparkle s2">✦</div>
        </div>
        <h3>No components yet</h3>
        <p>Add <code>#[component]</code> to your functions to see them here</p>
      </div>
    `;
    return;
  }
  
  el.innerHTML = list.map(c => `
    <div class="comp-card">
      <div class="comp-header">
        <span class="comp-name">${c.name}</span>
        <code class="comp-src">${c.file}:${c.line}</code>
      </div>
      ${c.doc ? `<p class="comp-doc">${c.doc}</p>` : ''}
      ${c.props?.length ? `
        <div class="comp-props">
          ${c.props.map(p => `
            <div class="prop-row">
              <span class="prop-name">${p.name}${p.required ? '<span class="req">*</span>' : ''}</span>
              <span class="prop-type">${p.ty}</span>
            </div>
          `).join('')}
        </div>
      ` : ''}
    </div>
  `).join('');
}

// =============================================================================
// WebSocket
// =============================================================================

function connectWS() {
  const ws = new WebSocket("ws://localhost:3001/__studio");
  ws.onopen = () => { State.wsConnected = true; };
  ws.onmessage = e => {
    try {
      const m = JSON.parse(e.data);
      if (m.type === "edit_result" && !m.success) toast("✗ " + m.error, true);
      if (m.type === "error") toast("✗ " + m.message, true);
    } catch (e) {}
  };
  ws.onclose = () => { State.wsConnected = false; setTimeout(connectWS, 2000); };
  State.ws = ws;
}

function sendWS(msg) {
  if (State.ws?.readyState === WebSocket.OPEN) State.ws.send(JSON.stringify(msg));
}

// =============================================================================
// Toast
// =============================================================================

function toast(msg, isError) {
  document.querySelectorAll(".toast").forEach(t => t.remove());
  const t = document.createElement("div");
  t.className = "toast" + (isError ? " error" : "");
  t.textContent = msg;
  document.body.appendChild(t);
  requestAnimationFrame(() => requestAnimationFrame(() => t.classList.add("show")));
  setTimeout(() => { t.classList.remove("show"); setTimeout(() => t.remove(), 200); }, 2000);
}

// =============================================================================
// Helpers
// =============================================================================

const $ = s => document.querySelector(s);
const $$ = s => document.querySelectorAll(s);

// =============================================================================
// Styles
// =============================================================================

function injectStyles() {
  const s = document.createElement("style");
  s.textContent = `
:root {
  --panel-width: 380px;
  
  /* Deep space background */
  --void: #07070a;
  --bg: #0c0c12;
  --bg2: #12121a;
  --bg3: #1a1a24;
  --bg4: #22222e;
  
  /* Subtle purple-tinted borders */
  --border: rgba(139, 133, 198, 0.12);
  --border-light: rgba(139, 133, 198, 0.2);
  --border-bright: rgba(139, 133, 198, 0.35);
  
  /* Text with slight warmth */
  --text: #f4f4f7;
  --text2: #a8a8b3;
  --text3: #6a6a78;
  
  /* Fun accent gradient */
  --accent1: #f0abfc;
  --accent2: #818cf8;
  --accent-glow: rgba(192, 148, 252, 0.4);
  
  /* Semantic colors */
  --green: #6ee7b7;
  --green-dim: rgba(110, 231, 183, 0.12);
  --red: #fca5a5;
  --yellow: #fcd34d;
  
  --radius: 12px;
  --radius-sm: 8px;
}

* { box-sizing: border-box; }

html, body {
  margin: 0; padding: 0; height: 100%; overflow: hidden;
  font: 13px/1.5 'Space Grotesk', system-ui, sans-serif;
  background: var(--void);
  color: var(--text2);
  -webkit-font-smoothing: antialiased;
}

body.resizing { cursor: ew-resize; user-select: none; }
body.resizing iframe { pointer-events: none; }
body.resizing * { transition: none !important; }

/* ==========================================================================
   Layout
   ========================================================================== */

#studio {
  display: flex; height: 100%;
  transition: padding 0.2s cubic-bezier(0.4, 0, 0.2, 1), 
              gap 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}
#studio.open { padding: 14px; gap: 14px; }

/* ==========================================================================
   Canvas - where the magic happens
   ========================================================================== */

#stage { flex: 1; min-width: 0; }

#canvas {
  width: 100%; height: 100%;
  border-radius: 0;
  overflow: hidden;
  position: relative;
  background: var(--void);
  transition: border-radius 0.2s ease, box-shadow 0.3s ease;
}

#studio.open #canvas {
  border-radius: 16px;
  box-shadow: 
    0 0 0 1px var(--border),
    0 4px 30px -5px rgba(0,0,0,0.5);
}

#iframe { 
  width: 100%; height: 100%; 
  border: none; 
  border-radius: inherit; 
  background: white;
}

/* Select mode - playful animated gradient border */
#studio.selecting #canvas {
  animation: glow-pulse 2s ease-in-out infinite;
}

/* Custom cursor for select mode */
#studio.selecting #iframe {
  cursor: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z" fill="%23f0abfc" stroke="%23ffffff" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>') 3 3, crosshair;
}

@keyframes glow-pulse {
  0%, 100% { 
    box-shadow: 
      0 0 0 2px var(--accent1),
      0 4px 30px -5px rgba(0,0,0,0.5),
      0 0 60px -10px var(--accent1);
  }
  50% { 
    box-shadow: 
      0 0 0 2px var(--accent2),
      0 4px 30px -5px rgba(0,0,0,0.5),
      0 0 60px -10px var(--accent2);
  }
}

/* ==========================================================================
   Highlight overlay
   ========================================================================== */

#highlight {
  position: fixed; 
  display: none; 
  pointer-events: none; 
  z-index: 99999;
  border: 2px solid var(--accent1);
  background: linear-gradient(135deg, rgba(240,171,252,0.1), rgba(129,140,248,0.1));
  border-radius: 4px;
  transition: all 60ms ease-out;
}

.highlight-label {
  position: absolute;
  top: -24px; left: -2px;
  padding: 2px 8px;
  background: linear-gradient(135deg, var(--accent1), var(--accent2));
  border-radius: 4px;
  font: 500 10px 'Space Grotesk', sans-serif;
  color: var(--void);
  text-transform: uppercase;
  letter-spacing: 0.03em;
  white-space: nowrap;
}

/* ==========================================================================
   Panel
   ========================================================================== */

#panel {
  width: 0; height: 100%;
  display: flex; flex-direction: column;
  background: linear-gradient(180deg, var(--bg) 0%, var(--bg2) 100%);
  border-radius: 16px;
  opacity: 0;
  overflow: hidden;
  transition: width 0.2s cubic-bezier(0.4, 0, 0.2, 1), 
              opacity 0.15s ease;
}

#studio.open #panel {
  width: var(--panel-width);
  opacity: 1;
  box-shadow: 
    inset 0 0 0 1px var(--border),
    0 4px 30px -5px rgba(0,0,0,0.3);
}

/* Resize handle */
#resize-handle {
  position: fixed;
  right: calc(var(--panel-width) + 14px);
  top: 14px;
  width: 14px; 
  height: calc(100% - 28px);
  cursor: ew-resize;
  z-index: 10000;
  display: none;
}
#studio.open #resize-handle { display: block; }
#resize-handle::before {
  content: '';
  position: absolute;
  left: 50%; top: 50%;
  transform: translate(-50%, -50%);
  width: 4px; height: 50px;
  background: transparent;
  border-radius: 4px;
  transition: background 0.15s;
}
#resize-handle:hover::before { background: var(--border-light); }
body.resizing #resize-handle::before { background: var(--accent1); }

/* ==========================================================================
   Header
   ========================================================================== */

#header {
  display: flex; 
  align-items: center; 
  gap: 12px;
  padding: 14px 16px;
  border-bottom: 1px solid var(--border);
}

#brand {
  display: flex;
  align-items: center;
  gap: 10px;
}

.brand-icon {
  width: 32px; height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, rgba(240,171,252,0.15), rgba(129,140,248,0.15));
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
}

#brand span {
  font-weight: 600;
  font-size: 15px;
  color: var(--text);
  letter-spacing: -0.02em;
}

#tools { flex: 1; }

.tool-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 7px 12px;
  background: var(--bg3);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  color: var(--text2);
  font: 500 12px 'Space Grotesk', sans-serif;
  cursor: pointer;
  transition: all 0.15s ease;
}
.tool-btn:hover { 
  background: var(--bg4);
  border-color: var(--border-light);
  color: var(--text);
}
.tool-btn.active {
  background: linear-gradient(135deg, rgba(240,171,252,0.2), rgba(129,140,248,0.2));
  border-color: var(--accent1);
  color: var(--accent1);
}
.tool-label { display: inline; }

#close-btn {
  width: 34px; height: 34px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: none;
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  color: var(--text3);
  cursor: pointer;
  transition: all 0.15s ease;
}
#close-btn:hover {
  background: var(--bg3);
  border-color: var(--border);
  color: var(--text2);
}

/* ==========================================================================
   Tabs
   ========================================================================== */

#tabs {
  display: flex;
  gap: 6px;
  padding: 12px 14px;
  border-bottom: 1px solid var(--border);
}

.tab {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 7px;
  padding: 10px 12px;
  background: none;
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  font: 500 12px 'Space Grotesk', sans-serif;
  color: var(--text3);
  cursor: pointer;
  transition: all 0.15s ease;
}
.tab svg { 
  opacity: 0.5; 
  transition: opacity 0.15s; 
}
.tab:hover { 
  color: var(--text2);
  background: var(--bg3);
}
.tab:hover svg { opacity: 0.7; }
.tab.active { 
  color: var(--text);
  background: var(--bg3);
  border-color: var(--border);
}
.tab.active svg { opacity: 1; }

/* ==========================================================================
   Body
   ========================================================================== */

#body {
  flex: 1; 
  overflow-y: auto; 
  padding: 16px;
}
#body::-webkit-scrollbar { width: 6px; }
#body::-webkit-scrollbar-track { background: transparent; }
#body::-webkit-scrollbar-thumb { 
  background: var(--border); 
  border-radius: 3px;
}
#body::-webkit-scrollbar-thumb:hover { 
  background: var(--border-light); 
}

.tab-panel { 
  display: none; 
  animation: slideIn 0.2s ease;
}
.tab-panel.active { display: block; }

@keyframes slideIn {
  from { opacity: 0; transform: translateY(8px); }
  to { opacity: 1; transform: translateY(0); }
}

/* ==========================================================================
   Footer
   ========================================================================== */

#footer {
  padding: 12px 16px;
  border-top: 1px solid var(--border);
}

.shortcut-hint {
  font: 11px 'Space Grotesk', sans-serif;
  color: var(--text3);
  text-align: center;
}

.shortcut-hint kbd {
  display: inline-block;
  padding: 2px 6px;
  margin: 0 1px;
  background: var(--bg3);
  border: 1px solid var(--border);
  border-radius: 4px;
  font: 10px 'JetBrains Mono', monospace;
  color: var(--text2);
}

/* ==========================================================================
   Empty States - whimsical and fun!
   ========================================================================== */

#inspect-empty, .empty-components {
  text-align: center;
  padding: 50px 24px;
}

.empty-visual {
  position: relative;
  display: inline-block;
  margin-bottom: 20px;
}

.cursor-icon {
  width: 72px; height: 72px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--bg3), var(--bg2));
  border: 1px dashed var(--border-light);
  border-radius: 20px;
  color: var(--text3);
  transition: all 0.3s ease;
}

.empty-visual:hover .cursor-icon {
  border-color: var(--accent1);
  color: var(--accent1);
  transform: scale(1.05) rotate(-3deg);
}

.sparkle {
  position: absolute;
  font-size: 14px;
  color: var(--accent1);
  opacity: 0;
  animation: sparkle 2s ease-in-out infinite;
}
.s1 { top: -5px; right: -5px; animation-delay: 0s; }
.s2 { bottom: 0; left: -8px; animation-delay: 0.5s; font-size: 12px; color: var(--accent2); }
.s3 { top: 50%; right: -12px; animation-delay: 1s; font-size: 18px; }

@keyframes sparkle {
  0%, 100% { opacity: 0; transform: scale(0.8); }
  50% { opacity: 1; transform: scale(1); }
}

#inspect-empty h3, .empty-components h3 {
  margin: 0 0 8px;
  font: 600 17px 'Space Grotesk', sans-serif;
  color: var(--text);
}

#inspect-empty p, .empty-components p {
  margin: 0;
  font-size: 13px;
  color: var(--text3);
  line-height: 1.6;
}

#inspect-empty kbd {
  display: inline-block;
  padding: 3px 8px;
  margin: 0 3px;
  background: var(--bg3);
  border: 1px solid var(--border);
  border-radius: 5px;
  font: 600 11px 'JetBrains Mono', monospace;
  color: var(--text2);
}

.empty-components code {
  padding: 2px 7px;
  background: var(--bg3);
  border-radius: 4px;
  font: 12px 'JetBrains Mono', monospace;
  color: var(--green);
}

/* ==========================================================================
   Inspect Panel
   ========================================================================== */

#inspect-content { display: none; }

.selected-card {
  background: var(--bg3);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 16px;
  margin-bottom: 20px;
}

.selected-header {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  margin-bottom: 10px;
}

.tag-badge {
  padding: 4px 10px;
  background: linear-gradient(135deg, rgba(240,171,252,0.15), rgba(129,140,248,0.15));
  border: 1px solid var(--border-light);
  border-radius: 6px;
  font: 600 13px 'JetBrains Mono', monospace;
  color: var(--accent1);
}

.id-badge {
  padding: 4px 10px;
  background: rgba(252, 211, 77, 0.12);
  border: 1px solid rgba(252, 211, 77, 0.2);
  border-radius: 6px;
  font: 600 12px 'JetBrains Mono', monospace;
  color: var(--yellow);
}

.component-pill {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: var(--green-dim);
  border: 1px solid rgba(110, 231, 183, 0.2);
  border-radius: var(--radius-sm);
  font: 500 12px 'Space Grotesk', sans-serif;
  color: var(--green);
  margin-bottom: 8px;
}

.source-line {
  display: flex;
  align-items: center;
  gap: 6px;
  font: 11px 'JetBrains Mono', monospace;
  color: var(--text3);
}

.section {
  margin-bottom: 16px;
}

.section-label {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
  font: 600 11px 'Space Grotesk', sans-serif;
  color: var(--text3);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}
.section-label svg { opacity: 0.6; }

#classes-input {
  width: 100%;
  min-height: 90px;
  padding: 14px;
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  font: 12px/1.7 'JetBrains Mono', monospace;
  color: var(--text);
  resize: vertical;
  transition: all 0.15s ease;
}
#classes-input::placeholder { color: var(--text3); }
#classes-input:focus {
  outline: none;
  border-color: var(--accent1);
  box-shadow: 0 0 0 3px rgba(240, 171, 252, 0.1);
}

#apply-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  margin-top: 12px;
  padding: 12px 20px;
  background: linear-gradient(135deg, var(--accent1), var(--accent2));
  border: none;
  border-radius: var(--radius);
  font: 600 13px 'Space Grotesk', sans-serif;
  color: var(--void);
  cursor: pointer;
  transition: all 0.2s ease;
}
#apply-btn:hover { 
  transform: translateY(-1px);
  box-shadow: 0 4px 20px -5px var(--accent-glow);
}
#apply-btn:active { transform: translateY(0) scale(0.98); }

/* ==========================================================================
   Tree
   ========================================================================== */

#tree { }

.tree-node {
  display: flex; 
  align-items: center; 
  gap: 8px;
  padding: 8px 12px;
  padding-left: calc(12px + var(--d) * 16px);
  border-radius: var(--radius-sm);
  font: 12px 'JetBrains Mono', monospace;
  cursor: pointer;
  transition: all 0.1s ease;
}
.tree-node:hover { background: var(--bg3); }
.tree-node.selected { 
  background: linear-gradient(135deg, rgba(240,171,252,0.1), rgba(129,140,248,0.1));
  box-shadow: inset 0 0 0 1px var(--border-light);
}
.tree-tag { color: var(--text); }
.tree-comp {
  margin-left: auto;
  padding: 2px 8px;
  background: var(--green-dim);
  border-radius: 4px;
  font: 500 10px 'Space Grotesk', sans-serif;
  color: var(--green);
}

.empty-msg {
  text-align: center;
  padding: 40px;
  color: var(--text3);
}

/* ==========================================================================
   Components
   ========================================================================== */

.comp-card {
  background: var(--bg3);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 16px;
  margin-bottom: 12px;
  transition: all 0.15s ease;
}
.comp-card:hover { 
  border-color: var(--border-light);
  transform: translateY(-2px);
  box-shadow: 0 8px 25px -10px rgba(0,0,0,0.3);
}

.comp-header { 
  display: flex; 
  justify-content: space-between; 
  align-items: baseline; 
  margin-bottom: 8px;
}
.comp-name { 
  font: 600 15px 'Space Grotesk', sans-serif;
  color: var(--green);
}
.comp-src { 
  font: 10px 'JetBrains Mono', monospace; 
  color: var(--text3);
}
.comp-doc { 
  margin: 10px 0 0; 
  font-size: 12px; 
  font-style: italic; 
  color: var(--text3);
  line-height: 1.6;
}
.comp-props { 
  margin-top: 14px; 
  padding-top: 14px; 
  border-top: 1px solid var(--border); 
}
.prop-row {
  display: flex; 
  justify-content: space-between;
  padding: 5px 0;
  font: 12px 'JetBrains Mono', monospace;
}
.prop-name { color: var(--text); }
.prop-name .req { color: var(--red); margin-left: 1px; }
.prop-type { color: var(--text3); }

/* ==========================================================================
   Toggle FAB
   ========================================================================== */

#toggle {
  position: fixed;
  bottom: 20px; right: 20px;
  width: 52px; height: 52px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--bg2), var(--bg));
  border: 1px solid var(--border);
  border-radius: 16px;
  color: var(--text2);
  cursor: pointer;
  z-index: 100000;
  transition: all 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  box-shadow: 
    0 4px 20px rgba(0,0,0,0.3),
    0 0 40px -15px var(--accent-glow);
}
#toggle:hover { 
  transform: scale(1.1) rotate(-5deg);
  border-color: var(--accent1);
  box-shadow: 
    0 8px 30px rgba(0,0,0,0.4),
    0 0 50px -10px var(--accent-glow);
}
#toggle:active { transform: scale(0.95); }
#studio.open #toggle { 
  opacity: 0; 
  pointer-events: none;
  transform: scale(0.8) rotate(10deg);
}

/* ==========================================================================
   Toast
   ========================================================================== */

.toast {
  position: fixed;
  bottom: 28px; left: 50%;
  transform: translateX(-50%) translateY(15px);
  padding: 12px 24px;
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  font: 500 13px 'Space Grotesk', sans-serif;
  color: var(--text);
  opacity: 0;
  z-index: 100001;
  transition: all 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  box-shadow: 0 10px 40px rgba(0,0,0,0.3);
}
.toast.show { 
  opacity: 1; 
  transform: translateX(-50%) translateY(0); 
}
.toast.error { 
  border-color: rgba(252, 165, 165, 0.3);
  color: var(--red);
}
  `;
  document.head.appendChild(s);
}

// =============================================================================
// Start
// =============================================================================

init();
