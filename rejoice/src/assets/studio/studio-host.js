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
  componentsOnPage: {}, // { componentName: [path1, path2, ...] }
  ws: null,
  wsConnected: false,
  activeTab: "inspect",
  iframe: null,
  previewIframe: null, // Second iframe for isolation mode
  panelWidth: 530,
  isResizing: false,
  shadowRoot: null, // Shadow root for panel isolation
  // Isolation mode
  isolatedComponent: null, // { name, meta, props: { propName: value, ... } }
};

const MIN_PANEL_WIDTH = 380;

// =============================================================================
// Design Tokens
// =============================================================================

const CSS_VARS = `
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
`;

// =============================================================================
// Init
// =============================================================================

function init() {
  document.head.insertAdjacentHTML(
    "beforeend",
    `
    <link href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  `,
  );

  // Main structure - panel content will be in shadow DOM
  document.body.innerHTML = `
    <div id="studio">
      <div id="stage">
        <div id="canvas">
          <iframe id="iframe" src="${getAppUrl()}"></iframe>
          <iframe id="preview-iframe"></iframe>
        </div>
      </div>
      
      <div id="highlight">
        <div class="highlight-label"></div>
      </div>
      
      <div id="resize-handle"></div>
      
      <div id="width-indicator">
        <div class="width-line"></div>
        <div class="width-label">0px</div>
      </div>
      
      <aside id="panel"></aside>
      
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

  // Inject light DOM styles (for stage, canvas, highlight, toggle)
  injectLightStyles();

  // Create shadow DOM for panel
  const panel = document.getElementById("panel");
  State.shadowRoot = panel.attachShadow({ mode: "open" });

  // Inject panel HTML and styles into shadow DOM
  State.shadowRoot.innerHTML = `
    <style>${getPanelStyles()}</style>
    <link href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    
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
          <line x1="8" y1="6" x2="21" y2="6"/>
          <line x1="8" y1="12" x2="21" y2="12"/>
          <line x1="8" y1="18" x2="21" y2="18"/>
          <line x1="3" y1="6" x2="3" y2="12"/>
          <line x1="3" y1="12" x2="6" y2="12"/>
          <line x1="3" y1="12" x2="3" y2="18"/>
          <line x1="3" y1="18" x2="6" y2="18"/>
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
  `;

  State.iframe = document.getElementById("iframe");
  State.previewIframe = document.getElementById("preview-iframe");
  document.documentElement.style.setProperty(
    "--panel-width",
    State.panelWidth + "px",
  );

  bindEvents();
  connectWS();
  fetchComponents();
}

function getAppUrl() {
  const params = new URLSearchParams(window.location.search);
  const path = params.get("path") || "/";
  return path + (path.includes("?") ? "&" : "?") + "__studio_bridge=1";
}

// =============================================================================
// Query Helpers
// =============================================================================

// Query light DOM (stage, canvas, highlight, toggle)
const $ = (s) => document.querySelector(s);
const $$ = (s) => document.querySelectorAll(s);

// Query shadow DOM (panel content)
const $panel = (s) => State.shadowRoot?.querySelector(s);
const $$panel = (s) => State.shadowRoot?.querySelectorAll(s);

// =============================================================================
// Events
// =============================================================================

function bindEvents() {
  $("#toggle").addEventListener("click", toggle);
  $panel("#close-btn").addEventListener("click", toggle);
  $panel("#select-btn").addEventListener("click", toggleSelect);

  $$panel(".tab").forEach((t) =>
    t.addEventListener("click", () => switchTab(t.dataset.tab)),
  );

  document.addEventListener("keydown", (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key === ".") {
      e.preventDefault();
      toggle();
    }
    if (e.key === "Escape") {
      if (State.isolatedComponent) {
        disableIsolation();
      } else if (State.selectMode) {
        toggleSelect();
      } else if (State.panelOpen) {
        toggle();
      }
    }
    // Check if focus is in an input (either in light DOM or shadow DOM)
    const activeEl = document.activeElement;
    const shadowActive = State.shadowRoot?.activeElement;
    const isInputFocused =
      activeEl?.matches("input,textarea") ||
      shadowActive?.matches("input,textarea");

    if (e.key === "s" && State.panelOpen && !isInputFocused) {
      e.preventDefault();
      toggleSelect();
    }
  });

  window.addEventListener("message", onMessage);
  State.iframe.addEventListener("load", onIframeLoad);

  // Resize
  const handle = $("#resize-handle");
  const widthIndicator = $("#width-indicator");
  const widthLabel = widthIndicator.querySelector(".width-label");

  handle.addEventListener("mousedown", (e) => {
    e.preventDefault();
    State.isResizing = true;
    document.body.classList.add("resizing");
    widthIndicator.classList.add("visible");
    updateWidthIndicator();
  });
  document.addEventListener("mousemove", (e) => {
    if (!State.isResizing) return;
    const w = Math.max(MIN_PANEL_WIDTH, window.innerWidth - e.clientX);
    State.panelWidth = w;
    document.documentElement.style.setProperty("--panel-width", w + "px");
    updateWidthIndicator();
  });
  document.addEventListener("mouseup", () => {
    if (State.isResizing) {
      State.isResizing = false;
      document.body.classList.remove("resizing");
      widthIndicator.classList.remove("visible");
    }
  });

  function updateWidthIndicator() {
    const canvas = $("#canvas");
    const canvasWidth = canvas.offsetWidth;
    widthLabel.textContent = `${canvasWidth}px`;
  }
}

function onIframeLoad() {
  // Sync URL
  try {
    const url = new URL(State.iframe.contentWindow.location.href);
    url.searchParams.delete("__studio_bridge");
    const path = url.pathname + url.search;
    history.replaceState(
      null,
      "",
      "/__studio" + (path !== "/" ? "?path=" + encodeURIComponent(path) : ""),
    );
  } catch (e) {}

  // If we were waiting for HMR, it's done now
  if (State.pendingToast) {
    dismissToast(State.pendingToast);
    State.pendingToast = null;
    showCanvasLoading(false);
    toast("Changes applied!", "success");
  }

  // Clear selected element since the page changed
  State.selectedElement = null;
  renderInspect();

  // Tree refresh is handled by bridge-ready message
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
  $panel("#select-btn").classList.toggle("active", State.selectMode);
  $("#studio").classList.toggle("selecting", State.selectMode);
  send({ type: "set-select-mode", enabled: State.selectMode });
  if (!State.selectMode) $("#highlight").style.display = "none";
}

function switchTab(tab) {
  State.activeTab = tab;
  $$panel(".tab").forEach((t) =>
    t.classList.toggle("active", t.dataset.tab === tab),
  );
  $$panel(".tab-panel").forEach((p) =>
    p.classList.toggle("active", p.dataset.tab === tab),
  );
  if (tab === "elements") {
    // Show loading state and request tree
    $panel("#tree").innerHTML = `<p class="empty-msg">Loading...</p>`;
    send({ type: "get-tree" });
  }
  if (tab === "components") {
    // Re-render to get latest on-page status
    renderComponents();
  }
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
    case "hover":
      showHighlight(m.rect, m.tagName, m.componentName, m.isComponentRoot);
      break;
    case "hover-end":
      hideHighlight();
      break;
    case "selected":
      onSelect(m);
      break;
    case "tree-data":
      renderTree(m.tree);
      break;
    case "navigate":
      onNavigate(m.path);
      break;
    case "shortcut":
      handleShortcut(m.key);
      break;
    case "bridge-ready":
      onBridgeReady();
      break;
    case "components-on-page":
      onComponentsOnPage(m.components);
      break;
  }
}

function handleShortcut(key) {
  if (key === "toggle") toggle();
  else if (key === "escape")
    State.selectMode ? toggleSelect() : State.panelOpen && toggle();
  else if (key === "select" && State.panelOpen) toggleSelect();
}

function onNavigate(path) {
  // Update URL
  history.replaceState(null, "", "/__studio?path=" + encodeURIComponent(path));

  // Clear selected element since the page changed
  State.selectedElement = null;
  renderInspect();

  // For SPA navigation, refresh data immediately (bridge is still loaded)
  if (State.activeTab === "elements") {
    send({ type: "get-tree" });
  }
  // Always refresh components on page for the Components tab
  send({ type: "get-components-on-page" });
}

function onBridgeReady() {
  // Bridge just loaded on new page - refresh data if needed
  if (State.activeTab === "elements") {
    send({ type: "get-tree" });
  }
  // Always refresh components on page
  send({ type: "get-components-on-page" });
}

function onComponentsOnPage(components) {
  State.componentsOnPage = components;
  // Always re-render components list to update button states
  renderComponents();
}

function showHighlight(r, tagName, componentName, isComponentRoot) {
  const h = $("#highlight");
  const c = $("#canvas").getBoundingClientRect();

  // Set position and size
  Object.assign(h.style, {
    display: "block",
    left: c.left + r.left + "px",
    top: c.top + r.top + "px",
    width: r.width + "px",
    height: r.height + "px",
  });

  // Set label based on context:
  // - Component root: just "ComponentName"
  // - Child inside component: "<tag> in ComponentName"
  // - Regular element: "<tag>"
  const label = h.querySelector(".highlight-label");
  if (componentName && isComponentRoot) {
    // This is the component itself
    label.textContent = componentName;
    h.classList.add("component");
  } else if (componentName) {
    // Child element inside a component
    label.textContent = `<${tagName}> in ${componentName}`;
    h.classList.add("component");
  } else {
    // Regular element, not in a component
    label.textContent = `<${tagName}>`;
    h.classList.remove("component");
  }
}

function hideHighlight() {
  const h = $("#highlight");
  h.style.display = "none";
  h.classList.remove("component");
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
    isComponentRoot: m.isComponentRoot,
    sourceLocation: m.sourceLocation,
    path: m.path,
  };

  if (State.selectMode) toggleSelect();
  renderInspect();
  if (State.activeTab !== "inspect") switchTab("inspect");
}

function renderInspect() {
  const el = State.selectedElement;
  const empty = $panel("#inspect-empty");
  const content = $panel("#inspect-content");

  if (!el) {
    empty.style.display = "";
    content.style.display = "none";
    content.innerHTML = "";
    return;
  }

  empty.style.display = "none";
  content.style.display = "block";

  // Check if this element is a component (or inside one)
  const isInComponent = !!el.componentName;
  const isComponentRoot = el.isComponentRoot;
  const isIsolated = State.isolatedComponent?.name === el.componentName;
  const comp = State.isolatedComponent;
  const meta = isInComponent
    ? State.components.find((c) => c.name === el.componentName)
    : null;
  const bgColor = isIsolated
    ? comp?.bgColor
    : getPreviewBgColor(el.componentName);

  // Build element header based on whether it's a component root or child
  let elementHeader = "";
  if (isComponentRoot) {
    // This IS the component - show component name prominently
    elementHeader = `
      <div class="element-header component-root">
        <div class="component-badge large">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="7" height="7" rx="1"/>
            <rect x="14" y="3" width="7" height="7" rx="1"/>
            <rect x="3" y="14" width="7" height="7" rx="1"/>
            <rect x="14" y="14" width="7" height="7" rx="1"/>
          </svg>
          ${el.componentName}
        </div>
        ${
          el.sourceLocation
            ? `
          <div class="source-link">
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14.5 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V7.5L14.5 2z"/>
              <polyline points="14,2 14,8 20,8"/>
            </svg>
            ${el.sourceLocation}
          </div>
        `
            : ""
        }
      </div>
    `;
  } else if (isInComponent) {
    // This is a child element inside a component
    elementHeader = `
      <div class="element-header">
        <div class="element-relationship">
          <span class="tag-badge">&lt;${el.tagName}&gt;</span>
          ${el.id ? `<span class="id-badge">#${el.id}</span>` : ""}
          <span class="inside-label">inside</span>
          <span class="component-badge">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="7" height="7" rx="1"/>
              <rect x="14" y="3" width="7" height="7" rx="1"/>
              <rect x="3" y="14" width="7" height="7" rx="1"/>
              <rect x="14" y="14" width="7" height="7" rx="1"/>
            </svg>
            ${el.componentName}
          </span>
        </div>
        ${
          el.sourceLocation
            ? `
          <div class="source-link">
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14.5 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V7.5L14.5 2z"/>
              <polyline points="14,2 14,8 20,8"/>
            </svg>
            ${el.sourceLocation}
          </div>
        `
            : ""
        }
      </div>
    `;
  } else {
    // Regular element, not in a component
    elementHeader = `
      <div class="element-header">
        <div class="element-tags">
          <span class="tag-badge">&lt;${el.tagName}&gt;</span>
          ${el.id ? `<span class="id-badge">#${el.id}</span>` : ""}
        </div>
      </div>
    `;
  }

  // Build the isolate panel section if this element is in a component
  let isolateSection = "";
  if (isInComponent) {
    // Determine styles label based on what we're editing
    const stylesTarget = isComponentRoot ? "component" : `<${el.tagName}>`;

    isolateSection = `
      <div class="panel-section isolate-section ${isIsolated ? "expanded" : ""}">
        <div class="panel-section-header" id="isolate-section-header">
          <div class="panel-section-title isolate-title">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="18" height="18" rx="2"/>
              <circle cx="12" cy="12" r="3"/>
            </svg>
            Isolate
          </div>
          <label class="prop-toggle small">
            <input type="checkbox" id="isolate-toggle" ${isIsolated ? "checked" : ""}>
            <span class="toggle-slider"></span>
          </label>
        </div>
        
        ${
          isIsolated && meta
            ? `
          <div class="panel-section-body">
            ${
              !isComponentRoot
                ? `
              <div class="isolate-note">
                Previewing the entire <strong>${el.componentName}</strong> component
              </div>
            `
                : ""
            }
            <div class="preview-controls">
              <div class="control-group">
                <label class="control-label">Background</label>
                <div class="bg-color-row">
                  <input type="color" id="preview-bg-color" value="${bgColor}">
                  <input type="text" id="preview-bg-hex" class="hex-input" value="${bgColor}" placeholder="#ffffff">
                  <div class="bg-presets">
                    <button class="bg-preset" data-color="#ffffff" title="White" style="background: #ffffff;"></button>
                    <button class="bg-preset" data-color="#f1f5f9" title="Light" style="background: #f1f5f9;"></button>
                    <button class="bg-preset" data-color="#1e293b" title="Dark" style="background: #1e293b;"></button>
                    <button class="bg-preset" data-color="#000000" title="Black" style="background: #000000;"></button>
                  </div>
                </div>
              </div>
              
              ${
                meta.props?.length
                  ? `
                <div class="control-group">
                  <label class="control-label">Props</label>
                  <div class="props-list">
                    ${meta.props.map((p) => renderPropEditor(p, comp.props[p.name])).join("")}
                  </div>
                </div>
              `
                  : ""
              }
            </div>
          </div>
        `
            : ""
        }
      </div>
    `;
  }

  // Styles section label changes based on context
  const stylesLabel = isComponentRoot
    ? `Classes <span class="styles-target">(on component)</span>`
    : isInComponent
      ? `Classes <span class="styles-target">(on &lt;${el.tagName}&gt;)</span>`
      : "Classes";

  content.innerHTML = `
    ${elementHeader}
    
    ${isolateSection}
    
    <div class="panel-section expanded">
      <div class="panel-section-header">
        <div class="panel-section-title">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M20.59 13.41l-7.17 7.17a2 2 0 01-2.83 0L2 12V2h10l8.59 8.59a2 2 0 010 2.82z"/>
            <line x1="7" y1="7" x2="7.01" y2="7"/>
          </svg>
          Styles
        </div>
      </div>
      <div class="panel-section-body">
        <div class="control-group">
          <label class="control-label">${stylesLabel}</label>
          <textarea id="classes-input" spellcheck="false" placeholder="flex items-center gap-4 ...">${el.classes || ""}</textarea>
          <button id="apply-btn" disabled>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <polyline points="20,6 9,17 4,12"/>
            </svg>
            Apply Changes
          </button>
        </div>
      </div>
    </div>
  `;

  // Bind isolate toggle if present
  const isolateToggle = $panel("#isolate-toggle");
  if (isolateToggle) {
    isolateToggle.addEventListener("change", () => {
      if (isolateToggle.checked) {
        enableIsolation(el.componentName);
      } else {
        disableIsolation();
      }
    });
  }

  // Bind background color picker if isolation is active
  if (isIsolated && isInComponent) {
    bindBgColorEvents();
    bindPropEditorEvents();
  }

  const applyBtn = $panel("#apply-btn");
  const classesInput = $panel("#classes-input");

  // Track what's saved in the filesystem (updates after successful sync)
  // Initialize to current classes if not already set
  if (el.savedClasses === undefined) {
    el.savedClasses = el.classes || "";
  }

  // Update button state and preview when input changes
  function onClassesInput() {
    const newClasses = classesInput.value;
    const hasChanges = newClasses !== el.savedClasses;
    applyBtn.disabled = !hasChanges;

    // Instant preview: update element in iframe immediately
    send({ type: "preview-classes", path: el.path, classes: newClasses });
  }

  classesInput.addEventListener("input", onClassesInput);

  // Apply = sync to filesystem
  applyBtn.addEventListener("click", () => {
    applyBtn.disabled = true; // Optimistically disable
    syncClassesToFile(classesInput.value);
  });

  classesInput.addEventListener("keydown", (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key === "Enter" && !applyBtn.disabled) {
      e.preventDefault();
      applyBtn.disabled = true; // Optimistically disable
      syncClassesToFile(classesInput.value);
    }
  });
}

// Sync classes to filesystem (called when Apply is clicked or cmd+enter)
function syncClassesToFile(classes) {
  const el = State.selectedElement;
  if (!el) return;

  const old = el.savedClasses || "";

  if (old) {
    // Store pending classes - will be committed on successful save
    State.pendingClassesSync = classes;

    // We have existing classes to search for
    if (el.sourceLocation) {
      // We have exact source location from #[component]
      // Format: /path/to/file.rs:line:column
      const parts = el.sourceLocation.split(":");
      const column = parts.pop(); // remove column
      const line = parts.pop(); // remove line
      const file = parts.join(":"); // rejoin in case path has colons (Windows)
      if (file && line) {
        State.pendingToast = toast("Saving changes...", "loading");
        showCanvasLoading(true);
        sendWS({
          type: "edit_file",
          file,
          edits: [
            {
              line: parseInt(line),
              old_text: `class="${old}"`,
              new_text: `class="${classes}"`,
            },
          ],
        });
      }
    } else {
      // No source location - search all files for the class string
      State.pendingToast = toast("Saving changes...", "loading");
      showCanvasLoading(true);
      sendWS({
        type: "edit_classes",
        old_classes: old,
        new_classes: classes,
        tag_hint: el.tagName,
      });
    }
  } else {
    // No existing classes - can't reliably find where to add them
    toast("Preview only (no existing classes)", "success");
  }
}

// =============================================================================
// Tree
// =============================================================================

// Track expanded state across re-renders
const expandedNodes = new Set(["0"]); // Root always expanded

function renderTree(tree) {
  const el = $panel("#tree");
  if (!tree) {
    el.innerHTML = `<p class="empty-msg">Loading...</p>`;
    return;
  }

  el.innerHTML = renderNode(tree, 0);
  bindTreeEvents(el);
}

function renderNode(n, depth) {
  if (!n) return "";

  const hasChildren = n.children && n.children.length > 0;
  const isExpanded = expandedNodes.has(n.path);
  const childCount = n.children?.length || 0;
  const classes = n.classes || [];

  // Build the node HTML
  let html = `
    <div class="tree-item" data-path="${n.path}">
      <div class="tree-row" style="--depth:${depth}">
        ${
          hasChildren
            ? `
          <button class="tree-toggle ${isExpanded ? "expanded" : ""}" data-path="${n.path}">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <polyline points="9,18 15,12 9,6"/>
            </svg>
          </button>
        `
            : `<span class="tree-toggle-spacer"></span>`
        }
        <span class="tree-tag">&lt;${n.tagName}${n.id ? `<span class="tree-id">#${n.id}</span>` : ""}&gt;</span>
        ${classes.length ? `<span class="tree-classes">.${classes.join(".")}</span>` : ""}
        ${n.componentName ? `<span class="tree-comp">${n.componentName}</span>` : ""}
        ${hasChildren && !isExpanded ? `<span class="tree-count">${childCount}</span>` : ""}
      </div>
      ${
        hasChildren
          ? `
        <div class="tree-children ${isExpanded ? "expanded" : ""}" style="--depth:${depth}">
          ${isExpanded ? n.children.map((c) => renderNode(c, depth + 1)).join("") : ""}
        </div>
      `
          : ""
      }
    </div>
  `;

  return html;
}

function bindTreeEvents(el) {
  // Toggle expand/collapse
  el.querySelectorAll(".tree-toggle").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      e.stopPropagation();
      const path = btn.dataset.path;

      if (expandedNodes.has(path)) {
        expandedNodes.delete(path);
      } else {
        expandedNodes.add(path);
      }
      // Re-fetch and re-render entire tree to update all states correctly
      send({ type: "get-tree" });
    });
  });

  // Select node on row click
  el.querySelectorAll(".tree-row").forEach((row) => {
    row.addEventListener("click", (e) => {
      if (e.target.closest(".tree-toggle")) return;
      e.stopPropagation();
      const path = row.closest(".tree-item").dataset.path;
      el.querySelectorAll(".tree-row").forEach((r) =>
        r.classList.remove("selected"),
      );
      row.classList.add("selected");
      send({ type: "select-by-path", path });
    });

    // Hover to highlight in preview
    row.addEventListener("mouseenter", () => {
      const path = row.closest(".tree-item").dataset.path;
      send({ type: "hover-by-path", path });
    });
    row.addEventListener("mouseleave", () => {
      send({ type: "hover-end-tree" });
    });
  });
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
  const el = $panel("#components");
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

  el.innerHTML = list
    .map((c) => {
      const onPage = State.componentsOnPage[c.name];
      const instanceCount = onPage?.length || 0;

      return `
      <div class="comp-card" data-component="${c.name}">
        <div class="comp-header">
          <span class="comp-name">${c.name}</span>
          <code class="comp-src">${c.file}:${c.line}</code>
        </div>
        ${c.doc ? `<p class="comp-doc">${c.doc}</p>` : ""}
        ${
          c.props?.length
            ? `
          <div class="comp-props">
            ${c.props
              .map(
                (p) => `
              <div class="prop-row">
                <span class="prop-name">${p.name}${p.required ? '<span class="req">*</span>' : ""}</span>
                <span class="prop-type">${p.ty}</span>
              </div>
            `,
              )
              .join("")}
          </div>
        `
            : ""
        }
        <div class="comp-actions">
          <button class="comp-btn isolate-btn" data-component="${c.name}" title="Open in isolation mode">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="3" y="3" width="18" height="18" rx="2"/>
              <path d="M9 3v18"/>
            </svg>
            Isolate
          </button>
          <button class="comp-btn reveal-btn" data-component="${c.name}" ${instanceCount === 0 ? "disabled" : ""} title="${instanceCount > 0 ? `${instanceCount} instance${instanceCount > 1 ? "s" : ""} on page` : "Not on current page"}">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="3"/>
              <path d="M12 2v4m0 12v4m10-10h-4M6 12H2m15.07-5.07l-2.83 2.83m-4.48 4.48l-2.83 2.83m0-10.14l2.83 2.83m4.48 4.48l2.83 2.83"/>
            </svg>
            Reveal${instanceCount > 0 ? ` (${instanceCount})` : ""}
          </button>
        </div>
      </div>
    `;
    })
    .join("");

  // Bind button events
  bindComponentEvents(el);
}

function bindComponentEvents(el) {
  // Reveal button - scroll to component on page
  el.querySelectorAll(".reveal-btn").forEach((btn) => {
    btn.addEventListener("click", () => {
      if (btn.disabled) return;
      const name = btn.dataset.component;
      send({ type: "reveal-component", name });
      // Switch to inspect tab to show the selected component
      switchTab("inspect");
    });
  });

  // Isolate button - open component in isolation mode
  el.querySelectorAll(".isolate-btn").forEach((btn) => {
    btn.addEventListener("click", () => {
      const name = btn.dataset.component;
      isolateComponent(name);
    });
  });
}

function enableIsolation(name) {
  // Find component metadata
  const meta = State.components.find((c) => c.name === name);
  if (!meta) {
    toast(`Component "${name}" not found in registry`, "error");
    return;
  }

  // Try to load saved props first, otherwise use defaults
  const savedProps = getPreviewProps(name);
  const props = {};

  if (meta.props) {
    meta.props.forEach((p) => {
      // Use saved value if available
      if (savedProps && savedProps[p.name] !== undefined) {
        props[p.name] = savedProps[p.name];
      } else if (p.default) {
        // Parse default value based on type
        props[p.name] = parseDefaultValue(p.default, p.ty);
      } else if (p.required) {
        // Set sensible defaults for required props without defaults
        props[p.name] = getDefaultForType(p.ty);
      }
    });
  }

  // Load saved background color for this component
  const bgColor = getPreviewBgColor(name);

  State.isolatedComponent = { name, meta, props, bgColor };

  // Show preview iframe, hide main iframe
  $("#studio").classList.add("isolated");

  // Load the component preview
  loadComponentPreview();

  // Re-render inspect panel to show isolation controls
  renderInspect();
}

function disableIsolation() {
  State.isolatedComponent = null;
  $("#studio").classList.remove("isolated");
  State.previewIframe.src = "";
  renderInspect();
}

// Legacy function for Components tab - now goes to inspect with isolation enabled
function isolateComponent(name) {
  // First, we need to select the component in the main iframe
  // Then enable isolation
  // For now, just enable isolation directly and switch to inspect tab
  enableIsolation(name);
  switchTab("inspect");
}

function loadComponentPreview() {
  if (!State.isolatedComponent) return;

  const { name, props, bgColor } = State.isolatedComponent;
  const params = new URLSearchParams();

  // Encode props as query params
  Object.entries(props).forEach(([key, value]) => {
    if (value !== undefined && value !== null) {
      params.set(`prop_${key}`, JSON.stringify(value));
    }
  });

  // Pass background color
  if (bgColor) {
    params.set("__bg", bgColor);
  }

  State.previewIframe.src = `/__studio/preview/${name}?${params.toString()}`;
}

function parseDefaultValue(defaultStr, ty) {
  // Handle common default value formats
  if (defaultStr === "true") return true;
  if (defaultStr === "false") return false;
  if (defaultStr === "None") return null;
  if (/^\d+$/.test(defaultStr)) return parseInt(defaultStr);
  if (/^\d+\.\d+$/.test(defaultStr)) return parseFloat(defaultStr);
  // For enum variants like "ButtonSize::Medium", extract just the variant
  if (defaultStr.includes("::")) return defaultStr.split("::").pop();
  // For string literals like "\"text\""
  if (defaultStr.startsWith('"') && defaultStr.endsWith('"')) {
    return defaultStr.slice(1, -1);
  }
  return defaultStr;
}

function getDefaultForType(ty) {
  // Provide sensible defaults for common types
  if (ty === "bool") return false;
  if (ty === "&str" || ty === "String") return "Example";
  if (
    ty === "i32" ||
    ty === "i64" ||
    ty === "u32" ||
    ty === "u64" ||
    ty === "usize"
  )
    return 0;
  if (ty === "f32" || ty === "f64") return 0.0;
  if (ty.startsWith("Option<")) return null;
  // For enum types, return empty string (will need to select)
  return "";
}

function bindBgColorEvents() {
  const colorInput = $panel("#preview-bg-color");
  const hexInput = $panel("#preview-bg-hex");
  const presets = $$panel(".bg-preset");

  function updateBgColor(color) {
    if (!State.isolatedComponent) return;
    State.isolatedComponent.bgColor = color;
    setPreviewBgColor(State.isolatedComponent.name, color);
    loadComponentPreview();

    // Keep inputs in sync
    if (colorInput) colorInput.value = color;
    if (hexInput) hexInput.value = color;
  }

  if (colorInput) {
    colorInput.addEventListener("input", () => updateBgColor(colorInput.value));
  }

  if (hexInput) {
    hexInput.addEventListener(
      "input",
      debounce(() => {
        const val = hexInput.value.trim();
        // Validate hex color
        if (/^#[0-9a-fA-F]{6}$/.test(val)) {
          updateBgColor(val);
        }
      }, 150),
    );
  }

  presets.forEach((btn) => {
    btn.addEventListener("click", () => {
      const color = btn.dataset.color;
      updateBgColor(color);
    });
  });
}

function renderPropEditor(propMeta, currentValue) {
  const { name, ty, required, default: defaultVal, doc } = propMeta;

  // Determine input type based on prop type
  let inputHtml = "";

  if (ty === "bool") {
    inputHtml = `
      <label class="prop-toggle">
        <input type="checkbox" data-prop="${name}" ${currentValue ? "checked" : ""}>
        <span class="toggle-slider"></span>
      </label>
    `;
  } else if (ty === "&str" || ty === "String") {
    inputHtml = `
      <input type="text" class="prop-input" data-prop="${name}" 
             value="${escapeHtml(currentValue || "")}" 
             placeholder="${defaultVal || "Enter text..."}">
    `;
  } else if (
    ty === "i32" ||
    ty === "i64" ||
    ty === "u32" ||
    ty === "u64" ||
    ty === "usize"
  ) {
    inputHtml = `
      <input type="number" class="prop-input" data-prop="${name}" 
             value="${currentValue ?? ""}" 
             placeholder="${defaultVal || "0"}">
    `;
  } else if (ty === "f32" || ty === "f64") {
    inputHtml = `
      <input type="number" step="0.1" class="prop-input" data-prop="${name}" 
             value="${currentValue ?? ""}" 
             placeholder="${defaultVal || "0.0"}">
    `;
  } else if (ty.startsWith("Option<")) {
    // Optional prop - add a checkbox to enable/disable
    const innerType = ty.slice(7, -1); // Extract type from Option<T>
    const isEnabled = currentValue !== null && currentValue !== undefined;
    inputHtml = `
      <div class="optional-prop">
        <label class="prop-toggle small">
          <input type="checkbox" data-prop-enabled="${name}" ${isEnabled ? "checked" : ""}>
          <span class="toggle-slider"></span>
        </label>
        <input type="text" class="prop-input" data-prop="${name}" 
               value="${escapeHtml(currentValue || "")}" 
               placeholder="None" ${!isEnabled ? "disabled" : ""}>
      </div>
    `;
  } else {
    // For enum types or unknown types, use text input
    inputHtml = `
      <input type="text" class="prop-input" data-prop="${name}" 
             value="${escapeHtml(currentValue || "")}" 
             placeholder="${defaultVal || ty}">
    `;
  }

  return `
    <div class="prop-editor-row">
      <div class="prop-editor-label">
        <span class="prop-editor-name">${name}${required ? '<span class="req">*</span>' : ""}</span>
        <span class="prop-editor-type">${ty}</span>
      </div>
      ${doc ? `<p class="prop-editor-doc">${doc}</p>` : ""}
      <div class="prop-editor-input">
        ${inputHtml}
      </div>
    </div>
  `;
}

function bindPropEditorEvents() {
  // Text/number inputs
  $$panel(".prop-input").forEach((input) => {
    input.addEventListener(
      "input",
      debounce(() => {
        const prop = input.dataset.prop;
        let value = input.value;

        // Convert to appropriate type
        if (input.type === "number") {
          value = input.value ? parseFloat(input.value) : null;
        }

        updateProp(prop, value);
      }, 150),
    );
  });

  // Boolean toggles
  $$panel('input[type="checkbox"][data-prop]').forEach((input) => {
    input.addEventListener("change", () => {
      updateProp(input.dataset.prop, input.checked);
    });
  });

  // Optional prop enable/disable
  $$panel("input[data-prop-enabled]").forEach((checkbox) => {
    checkbox.addEventListener("change", () => {
      const propName = checkbox.dataset.propEnabled;
      const valueInput = $panel(`input[data-prop="${propName}"]`);
      if (valueInput) {
        valueInput.disabled = !checkbox.checked;
        if (!checkbox.checked) {
          updateProp(propName, null);
        } else {
          updateProp(propName, valueInput.value || "");
        }
      }
    });
  });
}

function updateProp(name, value) {
  if (!State.isolatedComponent) return;
  State.isolatedComponent.props[name] = value;
  // Save props to localStorage
  setPreviewProps(State.isolatedComponent.name, State.isolatedComponent.props);
  loadComponentPreview();
}

function debounce(fn, delay) {
  let timeout;
  return (...args) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => fn(...args), delay);
  };
}

function escapeHtml(str) {
  if (typeof str !== "string") return str;
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

// =============================================================================
// Preview Background Color Storage
// =============================================================================

const PREVIEW_BG_STORAGE_KEY = "rejoice-studio-preview-bg";
const PREVIEW_PROPS_STORAGE_KEY = "rejoice-studio-preview-props";

function getPreviewBgColor(componentName) {
  try {
    const stored = localStorage.getItem(PREVIEW_BG_STORAGE_KEY);
    if (stored) {
      const colors = JSON.parse(stored);
      return colors[componentName] || "#ffffff";
    }
  } catch (e) {}
  return "#ffffff";
}

function setPreviewBgColor(componentName, color) {
  try {
    const stored = localStorage.getItem(PREVIEW_BG_STORAGE_KEY);
    const colors = stored ? JSON.parse(stored) : {};
    colors[componentName] = color;
    localStorage.setItem(PREVIEW_BG_STORAGE_KEY, JSON.stringify(colors));
  } catch (e) {}
}

function getPreviewProps(componentName) {
  try {
    const stored = localStorage.getItem(PREVIEW_PROPS_STORAGE_KEY);
    if (stored) {
      const allProps = JSON.parse(stored);
      return allProps[componentName] || null;
    }
  } catch (e) {}
  return null;
}

function setPreviewProps(componentName, props) {
  try {
    const stored = localStorage.getItem(PREVIEW_PROPS_STORAGE_KEY);
    const allProps = stored ? JSON.parse(stored) : {};
    allProps[componentName] = props;
    localStorage.setItem(PREVIEW_PROPS_STORAGE_KEY, JSON.stringify(allProps));
  } catch (e) {}
}

// =============================================================================
// WebSocket
// =============================================================================

function connectWS() {
  const ws = new WebSocket("ws://localhost:3001/__studio");
  ws.onopen = () => {
    State.wsConnected = true;
  };
  ws.onerror = (e) => {
    console.error("[Studio] WebSocket error:", e);
  };
  ws.onmessage = (e) => {
    try {
      const m = JSON.parse(e.data);
      if (m.type === "edit_result") {
        // Dismiss loading toast
        dismissToast(State.pendingToast);
        State.pendingToast = null;

        if (m.success) {
          // Commit the pending classes - they're now saved in the filesystem
          if (State.pendingClassesSync !== undefined && State.selectedElement) {
            State.selectedElement.savedClasses = State.pendingClassesSync;
            State.selectedElement.classes = State.pendingClassesSync;
            State.pendingClassesSync = undefined;
          }
          // Show compiling toast - will be replaced when HMR completes
          State.pendingToast = toast("Recompiling...", "loading");
        } else {
          State.pendingClassesSync = undefined;
          showCanvasLoading(false);
          toast(m.error || "Failed to save", "error");
        }
      }
      if (m.type === "file_updated") {
        // HMR completed
        dismissToast(State.pendingToast);
        State.pendingToast = null;
        showCanvasLoading(false);
        toast("Changes applied!", "success");
      }
      if (m.type === "error") {
        dismissToast(State.pendingToast);
        State.pendingToast = null;
        showCanvasLoading(false);
        toast(m.message || "An error occurred", "error");
      }
    } catch (e) {}
  };
  ws.onclose = () => {
    State.wsConnected = false;
    setTimeout(connectWS, 2000);
  };
  State.ws = ws;
}

function sendWS(msg) {
  if (State.ws?.readyState === WebSocket.OPEN)
    State.ws.send(JSON.stringify(msg));
}

// =============================================================================
// Toast
// =============================================================================

function toast(msg, type = "success") {
  // Remove existing toasts
  document.querySelectorAll(".toast").forEach((t) => {
    t.classList.remove("show");
    setTimeout(() => t.remove(), 200);
  });

  const t = document.createElement("div");
  t.className = `toast ${type}`;

  // Icon based on type
  const icons = {
    success: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20,6 9,17 4,12"/></svg>`,
    error: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>`,
    loading: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="spin"><path d="M21 12a9 9 0 11-6.219-8.56"/></svg>`,
  };

  t.innerHTML = `
    <span class="toast-icon">${icons[type] || icons.success}</span>
    <span class="toast-msg">${msg}</span>
  `;

  document.body.appendChild(t);
  requestAnimationFrame(() =>
    requestAnimationFrame(() => t.classList.add("show")),
  );

  // Loading toasts stay until dismissed
  if (type !== "loading") {
    setTimeout(() => {
      t.classList.remove("show");
      setTimeout(() => t.remove(), 200);
    }, 2500);
  }

  return t; // Return so loading toasts can be dismissed
}

// Dismiss a specific toast
function dismissToast(t) {
  if (t && t.parentNode) {
    t.classList.remove("show");
    setTimeout(() => t.remove(), 200);
  }
}

// Show loading state on canvas
function showCanvasLoading(show) {
  const canvas = $("#canvas");
  canvas.classList.toggle("loading", show);
}

// =============================================================================
// Light DOM Styles (stage, canvas, highlight, toggle, toast)
// =============================================================================

function injectLightStyles() {
  const s = document.createElement("style");
  s.textContent = `
:root {
  --panel-width: 380px;
  ${CSS_VARS}
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
  transition: opacity 0.2s ease;
}

/* Preview iframe for isolation mode */
#preview-iframe {
  position: absolute;
  inset: 0;
  width: 100%; height: 100%;
  border: none;
  border-radius: inherit;
  background: white;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.2s ease;
}

/* Isolation mode - swap iframes */
#studio.isolated #iframe {
  opacity: 0;
  pointer-events: none;
}

#studio.isolated #preview-iframe {
  opacity: 1;
  pointer-events: auto;
}

/* Isolated canvas has special border */
#studio.isolated #canvas {
  box-shadow: 
    0 0 0 2px var(--green),
    0 4px 30px -5px rgba(0,0,0,0.5),
    0 0 40px -10px rgba(110, 231, 183, 0.3);
}

/* Loading state */
#canvas.loading::after {
  content: '';
  position: absolute;
  inset: 0;
  background: rgba(7, 7, 10, 0.7);
  backdrop-filter: blur(2px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
  animation: fadeIn 0.15s ease;
}

#canvas.loading::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 32px;
  height: 32px;
  margin: -16px 0 0 -16px;
  border: 3px solid var(--border-light);
  border-top-color: var(--accent1);
  border-radius: 50%;
  z-index: 11;
  animation: spin 0.8s linear infinite;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes spin {
  to { transform: rotate(360deg); }
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

/* Component highlight - green instead of pink */
#highlight.component {
  border-color: var(--green);
  background: linear-gradient(135deg, rgba(110,231,183,0.1), rgba(52,211,153,0.1));
}

#highlight.component .highlight-label {
  background: linear-gradient(135deg, #6ee7b7, #34d399);
  color: #064e3b;
}

/* ==========================================================================
   Panel (host element only - content is in shadow DOM)
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

/* Width indicator */
#width-indicator {
  position: fixed;
  left: 14px;
  right: calc(var(--panel-width) + 28px);
  top: 50%;
  transform: translateY(-50%);
  display: none;
  align-items: center;
  justify-content: center;
  pointer-events: none;
  z-index: 10001;
  opacity: 0;
  transition: opacity 0.15s ease;
}

#width-indicator.visible {
  display: flex;
  opacity: 1;
}

.width-line {
  position: absolute;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, var(--accent1), var(--accent2));
}

.width-line::before,
.width-line::after {
  content: '';
  position: absolute;
  top: -4px;
  width: 1px;
  height: 9px;
  background: linear-gradient(180deg, var(--accent1), var(--accent2));
}

.width-line::before { left: 0; }
.width-line::after { right: 0; }

.width-label {
  position: relative;
  padding: 6px 12px;
  background: linear-gradient(135deg, var(--bg2), var(--bg));
  border: 1px solid var(--border-light);
  border-radius: 8px;
  font: 600 13px 'JetBrains Mono', monospace;
  color: var(--text);
  box-shadow: 0 4px 20px rgba(0,0,0,0.4);
}

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
  transform: translateX(-50%) translateY(20px) scale(0.95);
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 20px;
  background: linear-gradient(135deg, var(--bg2), var(--bg));
  border: 1px solid var(--border-light);
  border-radius: 14px;
  font: 500 13px 'Space Grotesk', sans-serif;
  color: var(--text);
  opacity: 0;
  z-index: 100001;
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  box-shadow: 
    0 10px 40px rgba(0,0,0,0.4),
    0 0 0 1px rgba(255,255,255,0.05) inset;
}

.toast.show { 
  opacity: 1; 
  transform: translateX(-50%) translateY(0) scale(1); 
}

.toast-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.toast-msg {
  white-space: nowrap;
}

/* Success toast */
.toast.success {
  border-color: rgba(110, 231, 183, 0.3);
}
.toast.success .toast-icon {
  color: var(--green);
}

/* Error toast */
.toast.error { 
  border-color: rgba(252, 165, 165, 0.3);
}
.toast.error .toast-icon {
  color: var(--red);
}
.toast.error .toast-msg {
  color: var(--red);
}

/* Loading toast */
.toast.loading {
  border-color: rgba(240, 171, 252, 0.3);
}
.toast.loading .toast-icon {
  color: var(--accent1);
}

.toast .spin {
  animation: spin 1s linear infinite;
}
  `;
  document.head.appendChild(s);
}

// =============================================================================
// Panel Styles (Shadow DOM - completely isolated from site CSS)
// =============================================================================

function getPanelStyles() {
  return `
:host {
  display: flex;
  flex-direction: column;
  height: 100%;
  font: 13px/1.5 'Space Grotesk', system-ui, sans-serif;
  color: #a8a8b3;
  -webkit-font-smoothing: antialiased;
  ${CSS_VARS}
}

* { box-sizing: border-box; }

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

/* Element header - shows tag, id, component name, source */
.element-header {
  padding: 14px 16px;
  background: var(--bg3);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  margin-bottom: 12px;
}

/* Component root header - more prominent */
.element-header.component-root {
  background: linear-gradient(135deg, var(--bg3) 0%, rgba(110, 231, 183, 0.05) 100%);
  border-color: rgba(110, 231, 183, 0.2);
}

.element-tags {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.element-header:not(.component-root) .element-tags {
  margin-bottom: 8px;
}

/* Element relationship row - for child elements inside components */
.element-relationship {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  margin-bottom: 8px;
}

.element-relationship .tag-badge,
.element-relationship .id-badge,
.element-relationship .component-badge {
  margin-bottom: 0;
}

.inside-label {
  font: 500 10px 'Space Grotesk', sans-serif;
  color: var(--text3);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.tag-badge {
  padding: 5px 10px;
  background: linear-gradient(135deg, rgba(240,171,252,0.12), rgba(129,140,248,0.12));
  border: 1px solid rgba(240,171,252,0.2);
  border-radius: 6px;
  font: 600 11px/1 'JetBrains Mono', monospace;
  color: var(--accent1);
}

.id-badge {
  padding: 5px 10px;
  background: rgba(252, 211, 77, 0.1);
  border: 1px solid rgba(252, 211, 77, 0.2);
  border-radius: 6px;
  font: 600 11px/1 'JetBrains Mono', monospace;
  color: var(--yellow);
}

.component-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 5px 10px;
  background: var(--green-dim);
  border: 1px solid rgba(110, 231, 183, 0.2);
  border-radius: 6px;
  font: 600 11px/1 'Space Grotesk', sans-serif;
  color: var(--green);
  margin-bottom: 6px;
}

.component-badge.large {
  padding: 6px 12px;
  font-size: 12px;
  margin-bottom: 8px;
}

.component-badge.large svg {
  width: 14px;
  height: 14px;
}

.source-link {
  display: flex;
  align-items: center;
  gap: 5px;
  font: 10px 'JetBrains Mono', monospace;
  color: var(--text3);
}

/* Panel sections - collapsible cards */
.panel-section {
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  margin-bottom: 12px;
  overflow: hidden;
}

.panel-section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 14px;
  background: var(--bg3);
  border-bottom: 1px solid transparent;
  cursor: default;
}

.panel-section.expanded .panel-section-header {
  border-bottom-color: var(--border);
}

.panel-section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font: 600 11px 'Space Grotesk', sans-serif;
  color: var(--text2);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.panel-section-title svg {
  opacity: 0.5;
}

.panel-section-body {
  display: none;
  padding: 14px;
}

.panel-section.expanded .panel-section-body {
  display: block;
}

/* Isolate section special styling - green title only */
.isolate-section .isolate-title {
  color: var(--green);
}

.isolate-section .isolate-title svg {
  opacity: 0.8;
}

/* Note shown when isolating a child element */
.isolate-note {
  padding: 10px 12px;
  margin-bottom: 14px;
  background: rgba(110, 231, 183, 0.06);
  border: 1px solid rgba(110, 231, 183, 0.15);
  border-radius: var(--radius-sm);
  font: 11px/1.5 'Space Grotesk', sans-serif;
  color: var(--text3);
}

.isolate-note strong {
  color: var(--green);
  font-weight: 600;
}

/* Styles target indicator */
.styles-target {
  font-weight: 400;
  color: var(--text3);
  text-transform: none;
  letter-spacing: normal;
}

/* Control groups within sections */
.control-group {
  margin-bottom: 16px;
}

.control-group:last-child {
  margin-bottom: 0;
}

.control-label {
  display: block;
  margin-bottom: 8px;
  font: 500 11px 'Space Grotesk', sans-serif;
  color: var(--text3);
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

/* Classes textarea */
#classes-input {
  width: 100%;
  min-height: 80px;
  padding: 12px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  font: 12px/1.6 'JetBrains Mono', monospace;
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
  gap: 6px;
  width: 100%;
  margin-top: 10px;
  padding: 10px 16px;
  background: linear-gradient(135deg, var(--accent1), var(--accent2));
  border: none;
  border-radius: var(--radius-sm);
  font: 600 12px 'Space Grotesk', sans-serif;
  color: var(--void);
  cursor: pointer;
  transition: all 0.2s ease;
}

#apply-btn:hover:not(:disabled) { 
  transform: translateY(-1px);
  box-shadow: 0 4px 20px -5px var(--accent-glow);
}

#apply-btn:active:not(:disabled) { 
  transform: translateY(0) scale(0.98); 
}

#apply-btn:disabled {
  opacity: 0.35;
  cursor: not-allowed;
}

/* ==========================================================================
   Tree
   ========================================================================== */

#tree {
  font: 12px 'JetBrains Mono', monospace;
}

.tree-item {
  /* Container for row + children */
}

.tree-row {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 5px 8px;
  padding-left: calc(8px + var(--depth) * 16px);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.1s ease;
  position: relative;
}

.tree-row:hover {
  background: var(--bg3);
}

.tree-row.selected {
  background: linear-gradient(135deg, rgba(240,171,252,0.12), rgba(129,140,248,0.12));
}

.tree-row.selected::before {
  content: '';
  position: absolute;
  left: 0;
  top: 4px;
  bottom: 4px;
  width: 3px;
  background: linear-gradient(180deg, var(--accent1), var(--accent2));
  border-radius: 0 2px 2px 0;
}

/* Toggle button */
.tree-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  padding: 0;
  background: none;
  border: none;
  border-radius: 4px;
  color: var(--text3);
  cursor: pointer;
  transition: all 0.15s ease;
  flex-shrink: 0;
}

.tree-toggle:hover {
  background: var(--bg4);
  color: var(--text2);
}

.tree-toggle svg {
  transition: transform 0.15s ease;
}

.tree-toggle.expanded svg {
  transform: rotate(90deg);
}

.tree-toggle-spacer {
  width: 18px;
  flex-shrink: 0;
}

/* Tag name */
.tree-tag {
  color: var(--accent1);
  font-weight: 500;
  white-space: nowrap;
}

.tree-id {
  color: var(--yellow);
  font-weight: 400;
}

/* Classes preview */
.tree-classes {
  color: var(--text3);
  font-size: 10px;
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  opacity: 0.8;
}

/* Component badge */
.tree-comp {
  margin-left: auto;
  padding: 2px 6px;
  background: var(--green-dim);
  border-radius: 4px;
  font: 500 9px 'Space Grotesk', sans-serif;
  color: var(--green);
  letter-spacing: 0.02em;
  flex-shrink: 0;
}

/* Child count badge */
.tree-count {
  padding: 1px 5px;
  background: var(--bg4);
  border-radius: 4px;
  font: 500 9px 'Space Grotesk', sans-serif;
  color: var(--text3);
  flex-shrink: 0;
}

/* Children container */
.tree-children {
  display: none;
  position: relative;
}

.tree-children.expanded {
  display: block;
}

/* Vertical connection line */
.tree-children.expanded::before {
  content: '';
  position: absolute;
  left: calc(17px + var(--depth, 0) * 16px);
  top: 0;
  bottom: 8px;
  width: 1px;
  background: var(--border-light);
  pointer-events: none;
}

.empty-msg {
  text-align: center;
  padding: 40px;
  color: var(--text3);
}

/* Empty state for tree */
#tree:empty::after {
  content: 'Loading...';
  display: block;
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
  background: none;
  padding: 0;
  border: none;
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

/* Component action buttons */
.comp-actions {
  display: flex;
  gap: 8px;
  margin-top: 14px;
  padding-top: 14px;
  border-top: 1px solid var(--border);
}

.comp-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 12px;
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  font: 500 11px 'Space Grotesk', sans-serif;
  color: var(--text2);
  cursor: pointer;
  transition: all 0.15s ease;
}

.comp-btn:hover:not(:disabled) {
  background: var(--bg4);
  border-color: var(--border-light);
  color: var(--text);
}

.comp-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.comp-btn.isolate-btn:hover:not(:disabled) {
  border-color: var(--accent1);
  color: var(--accent1);
}

.comp-btn.reveal-btn:hover:not(:disabled) {
  border-color: var(--green);
  color: var(--green);
}

/* ==========================================================================
   Preview Controls (in inspect panel)
   ========================================================================== */

/* Preview controls */
.preview-controls {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.bg-color-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.bg-color-row input[type="color"] {
  width: 36px;
  height: 36px;
  padding: 0;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  background: none;
  cursor: pointer;
  overflow: hidden;
  flex-shrink: 0;
}

.bg-color-row input[type="color"]::-webkit-color-swatch-wrapper {
  padding: 3px;
}

.bg-color-row input[type="color"]::-webkit-color-swatch {
  border: none;
  border-radius: 4px;
}

.hex-input {
  width: 80px;
  padding: 8px 10px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  font: 11px 'JetBrains Mono', monospace;
  color: var(--text);
  flex-shrink: 0;
}

.hex-input:focus {
  outline: none;
  border-color: var(--accent1);
}

.bg-presets {
  display: flex;
  gap: 4px;
  margin-left: auto;
}

.bg-preset {
  width: 22px;
  height: 22px;
  padding: 0;
  border: 1px solid var(--border);
  border-radius: 5px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.bg-preset:hover {
  transform: scale(1.15);
  border-color: var(--border-light);
}

.bg-preset[data-color="#ffffff"],
.bg-preset[data-color="#f1f5f9"] {
  border-color: var(--border-light);
}

/* Props list in preview */
.props-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.no-props-msg {
  text-align: center;
  padding: 30px 20px;
  color: var(--text3);
}

.no-props-msg p {
  margin: 0;
  font-size: 13px;
}

/* Props editor */
.props-editor {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.prop-editor-row {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: 12px;
}

.prop-editor-label {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 6px;
}

.prop-editor-name {
  font: 500 12px 'Space Grotesk', sans-serif;
  color: var(--text);
}

.prop-editor-name .req {
  color: var(--red);
  margin-left: 2px;
}

.prop-editor-type {
  font: 10px 'JetBrains Mono', monospace;
  color: var(--text3);
}

.prop-editor-doc {
  margin: 0 0 8px;
  font-size: 10px;
  color: var(--text3);
  font-style: italic;
  line-height: 1.4;
}

.prop-editor-input {
  display: flex;
  align-items: center;
}

.prop-input {
  width: 100%;
  padding: 8px 10px;
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: 6px;
  font: 11px 'JetBrains Mono', monospace;
  color: var(--text);
  transition: all 0.15s ease;
}

.prop-input::placeholder {
  color: var(--text3);
}

.prop-input:focus {
  outline: none;
  border-color: var(--green);
  box-shadow: 0 0 0 3px rgba(110, 231, 183, 0.1);
}

.prop-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Boolean toggle */
.prop-toggle {
  position: relative;
  display: inline-flex;
  width: 44px;
  height: 24px;
  cursor: pointer;
}

.prop-toggle.small {
  width: 36px;
  height: 20px;
  flex-shrink: 0;
}

.prop-toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  inset: 0;
  background: var(--bg4);
  border: 1px solid var(--border);
  border-radius: 12px;
  transition: all 0.2s ease;
}

.toggle-slider::before {
  content: '';
  position: absolute;
  left: 3px;
  top: 50%;
  transform: translateY(-50%);
  width: 16px;
  height: 16px;
  background: var(--text3);
  border-radius: 50%;
  transition: all 0.2s ease;
}

.prop-toggle.small .toggle-slider::before {
  width: 14px;
  height: 14px;
  left: 2px;
}

.prop-toggle input:checked + .toggle-slider {
  background: linear-gradient(135deg, var(--accent1), var(--accent2));
  border-color: transparent;
}

.prop-toggle input:checked + .toggle-slider::before {
  background: var(--void);
  transform: translateY(-50%) translateX(20px);
}

.prop-toggle.small input:checked + .toggle-slider::before {
  transform: translateY(-50%) translateX(16px);
}

/* Optional prop with enable toggle */
.optional-prop {
  display: flex;
  align-items: center;
  gap: 10px;
}

.optional-prop .prop-input {
  flex: 1;
}
  `;
}

// =============================================================================
// Start
// =============================================================================

init();
