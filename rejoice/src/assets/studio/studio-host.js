/**
 * Rejoice Studio
 * 
 * A minimal, functional visual development environment.
 * Nothing extra. Everything purposeful.
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
  panelWidth: 320,
  isResizing: false,
};

const MIN_PANEL_WIDTH = 380;

// =============================================================================
// Init
// =============================================================================

function init() {
  document.head.insertAdjacentHTML('beforeend', `
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  `);
  
  document.body.innerHTML = `
    <div id="studio">
      <div id="stage">
        <div id="canvas">
          <iframe id="iframe" src="${getAppUrl()}"></iframe>
        </div>
      </div>
      
      <div id="highlight"></div>
      
      <div id="resize-handle"></div>
      <aside id="panel">
        
        <header id="header">
          <button id="close-btn" title="Close (Esc)">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
              <path d="M18 6L6 18M6 6l12 12"/>
            </svg>
          </button>
          
          <nav id="tabs">
            <button class="tab active" data-tab="inspect">Inspect</button>
            <button class="tab" data-tab="elements">Elements</button>
            <button class="tab" data-tab="components">Components</button>
          </nav>
          
          <button id="select-btn" title="Select element (S)">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z"/>
            </svg>
          </button>
        </header>
        
        <main id="body">
          <section class="tab-panel active" data-tab="inspect">
            <div id="inspect-empty">
              <p>Select an element to inspect</p>
              <p class="hint">Press <kbd>S</kbd> then click any element</p>
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
      </aside>
      
      <button id="toggle" title="Open Studio (Cmd+.)">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 2L2 7l10 5 10-5-10-5z"/>
          <path d="M2 17l10 5 10-5"/>
          <path d="M2 12l10 5 10-5"/>
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
    case "hover": showHighlight(m.rect); break;
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

function showHighlight(r) {
  const h = $("#highlight");
  const c = $("#canvas").getBoundingClientRect();
  Object.assign(h.style, {
    display: "block",
    left: c.left + r.left + "px",
    top: c.top + r.top + "px",
    width: r.width + "px",
    height: r.height + "px",
  });
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
    <div class="inspect-el">
      <code class="el-tag">&lt;${el.tagName}${el.id ? ` id="${el.id}"` : ''}&gt;</code>
      ${el.componentName ? `<span class="el-component">${el.componentName}</span>` : ''}
      ${el.sourceLocation ? `<span class="el-source">${el.sourceLocation}</span>` : ''}
    </div>
    
    <label class="field-label">Classes</label>
    <textarea id="classes-input" spellcheck="false">${el.classes || ''}</textarea>
    <button id="apply-btn">Apply</button>
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
  
  toast("Applied");
}

// =============================================================================
// Tree
// =============================================================================

function renderTree(tree) {
  const el = $("#tree");
  if (!tree) { el.innerHTML = `<p class="empty">Loading...</p>`; return; }
  
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
    el.innerHTML = `<p class="empty">No components registered.<br>Use #[component] to add them.</p>`;
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
              <span>${p.name}${p.required ? '*' : ''}</span>
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
      if (m.type === "edit_result" && !m.success) toast("Error: " + m.error, true);
      if (m.type === "error") toast(m.message, true);
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
  setTimeout(() => { t.classList.remove("show"); setTimeout(() => t.remove(), 150); }, 1800);
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
  --panel: 320px;
  --bg: #111;
  --bg2: #191919;
  --bg3: #222;
  --border: #2a2a2a;
  --text: #e5e5e5;
  --text2: #888;
  --text3: #555;
  --accent: #888;
  --radius: 8px;
}

* { box-sizing: border-box; }

html, body {
  margin: 0; padding: 0; height: 100%; overflow: hidden;
  font: 13px/1.5 'Inter', system-ui, sans-serif;
  background: #0a0a0a;
  color: var(--text2);
  -webkit-font-smoothing: antialiased;
}

body.resizing { cursor: ew-resize; user-select: none; }
body.resizing iframe { pointer-events: none; }
body.resizing #studio,
body.resizing #panel,
body.resizing #canvas,
body.resizing #resize-handle { transition: none; }

/* Layout */
#studio {
  display: flex; height: 100%;
  transition: padding 0.15s ease-out, gap 0.15s ease-out;
}
#studio.open { padding: 12px; gap: 12px; }

/* Stage */
#stage { flex: 1; min-width: 0; }

#canvas {
  width: 100%; height: 100%;
  border-radius: 0;
  overflow: hidden;
  transition: border-radius 0.15s ease-out, box-shadow 0.15s ease-out;
}
#studio.open #canvas {
  border-radius: 10px;
  box-shadow: 0 0 0 1px var(--border), 0 20px 50px -20px rgba(0,0,0,0.7);
}

#iframe { width: 100%; height: 100%; border: none; border-radius: inherit; }

/* Select mode */
#studio.selecting #canvas {
  box-shadow: 0 0 0 2px var(--accent), 0 20px 50px -20px rgba(0,0,0,0.7);
}

/* Highlight */
#highlight {
  position: fixed; display: none; pointer-events: none; z-index: 99999;
  border: 1.5px solid var(--accent);
  background: rgba(136,136,136,0.08);
  border-radius: 2px;
  transition: all 50ms ease-out;
}

/* Panel */
#panel {
  width: 0; height: 100%;
  display: flex; flex-direction: column;
  background: var(--bg);
  border-radius: 10px;
  opacity: 0;
  overflow: hidden;
  transition: width 0.15s ease-out, opacity 0.1s ease-out;
}
#studio.open #panel {
  width: var(--panel-width);
  opacity: 1;
  border: 1px solid var(--border);
}
body.resizing #panel {
  transition: none;
}

#resize-handle {
  position: fixed;
  right: calc(var(--panel-width) + 12px);
  top: 12px;
  width: 12px; 
  height: calc(100% - 24px);
  cursor: ew-resize;
  z-index: 10000;
  display: none;
}
#studio.open #resize-handle {
  display: block;
}
#resize-handle::before {
  content: '';
  position: absolute;
  left: 50%; top: 50%;
  transform: translate(-50%, -50%);
  width: 4px; height: 48px;
  background: transparent;
  border-radius: 4px;
}
#resize-handle:hover::before {
  background: var(--text3);
}
body.resizing #resize-handle::before {
  background: var(--text2);
}

/* Header */
#header {
  display: flex; align-items: center; gap: 8px;
  padding: 10px 12px;
  border-bottom: 1px solid var(--border);
}

#close-btn, #select-btn {
  width: 30px; height: 30px;
  display: flex; align-items: center; justify-content: center;
  background: none; border: none; border-radius: 6px;
  color: var(--text3);
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
}
#close-btn:hover, #select-btn:hover { background: var(--bg3); color: var(--text2); }
#select-btn.active { background: var(--bg3); color: var(--text); }

#tabs { display: flex; flex: 1; gap: 2px; }

.tab {
  flex: 1;
  padding: 6px 0;
  background: none; border: none; border-radius: 5px;
  font: inherit; font-size: 12px;
  color: var(--text3);
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
}
.tab:hover { color: var(--text2); }
.tab.active { background: var(--bg3); color: var(--text); }

/* Body */
#body {
  flex: 1; overflow-y: auto; padding: 12px;
}
#body::-webkit-scrollbar { width: 6px; }
#body::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }

.tab-panel { display: none; }
.tab-panel.active { display: block; }

/* Empty states */
#inspect-empty, .empty {
  text-align: center;
  padding: 32px 16px;
  color: var(--text3);
}
#inspect-empty p:first-child { margin: 0 0 8px; color: var(--text2); }
.hint { font-size: 12px; }
kbd {
  display: inline-block;
  padding: 2px 6px;
  background: var(--bg3);
  border: 1px solid var(--border);
  border-radius: 4px;
  font: 11px var(--font-mono, 'JetBrains Mono', monospace);
}

/* Inspect */
#inspect-content { display: none; }

.inspect-el {
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 10px 12px;
  margin-bottom: 12px;
}

.el-tag {
  display: block;
  font: 13px/1.4 'JetBrains Mono', monospace;
  color: var(--text);
}

.el-component {
  display: inline-block;
  margin-top: 6px;
  padding: 2px 8px;
  background: rgba(80,200,120,0.12);
  border-radius: 4px;
  font-size: 11px;
  color: rgb(80,200,120);
}

.el-source {
  display: block;
  margin-top: 6px;
  font: 11px 'JetBrains Mono', monospace;
  color: var(--text3);
}

.field-label {
  display: block;
  margin-bottom: 6px;
  font-size: 11px;
  font-weight: 500;
  color: var(--text3);
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

#classes-input {
  width: 100%;
  min-height: 72px;
  padding: 10px;
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  font: 12px/1.6 'JetBrains Mono', monospace;
  color: var(--text);
  resize: vertical;
}
#classes-input:focus {
  outline: none;
  border-color: var(--text3);
}

#apply-btn {
  width: 100%;
  margin-top: 8px;
  padding: 10px;
  background: var(--bg3);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  font: inherit;
  color: var(--text);
  cursor: pointer;
  transition: background 0.15s;
}
#apply-btn:hover { background: #2a2a2a; }

/* Tree */
.tree-node {
  display: flex; align-items: center; gap: 6px;
  padding: 5px 8px;
  padding-left: calc(8px + var(--d) * 12px);
  border-radius: 5px;
  font: 12px 'JetBrains Mono', monospace;
  cursor: pointer;
  transition: background 0.1s;
}
.tree-node:hover { background: var(--bg3); }
.tree-node.selected { background: rgba(136,136,136,0.15); }
.tree-tag { color: var(--text); }
.tree-comp {
  margin-left: auto;
  padding: 1px 6px;
  background: rgba(80,200,120,0.12);
  border-radius: 3px;
  font-size: 10px;
  color: rgb(80,200,120);
}

/* Components */
.comp-card {
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 12px;
  margin-bottom: 8px;
}
.comp-header { display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 4px; }
.comp-name { font-weight: 600; color: rgb(80,200,120); }
.comp-src { font: 10px 'JetBrains Mono', monospace; color: var(--text3); }
.comp-doc { margin: 8px 0 0; font-size: 12px; font-style: italic; color: var(--text3); }
.comp-props { margin-top: 10px; padding-top: 10px; border-top: 1px solid var(--border); }
.prop-row {
  display: flex; justify-content: space-between;
  padding: 3px 0;
  font: 12px 'JetBrains Mono', monospace;
}
.prop-row span:first-child { color: var(--text); }
.prop-type { color: var(--text3); }

/* Toggle */
#toggle {
  position: fixed;
  bottom: 16px; right: 16px;
  width: 44px; height: 44px;
  display: flex; align-items: center; justify-content: center;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 10px;
  color: var(--text2);
  cursor: pointer;
  z-index: 100000;
  transition: transform 0.15s ease, background 0.15s ease, opacity 0.2s ease;
}
#toggle:hover { background: var(--bg2); transform: scale(1.05); }
#toggle:active { transform: scale(0.97); }
#studio.open #toggle { opacity: 0; pointer-events: none; }

/* Toast */
.toast {
  position: fixed;
  bottom: 20px; left: 50%;
  transform: translateX(-50%) translateY(8px);
  padding: 10px 18px;
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  font-size: 13px;
  color: var(--text);
  opacity: 0;
  transition: opacity 0.15s, transform 0.15s ease;
  z-index: 100001;
}
.toast.show { opacity: 1; transform: translateX(-50%) translateY(0); }
.toast.error { border-color: #a33; color: #f88; }
  `;
  document.head.appendChild(s);
}

// =============================================================================
// Start
// =============================================================================

init();
