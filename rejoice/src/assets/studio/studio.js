/**
 * Rejoice Studio - Visual development environment
 * 
 * Web Components-based overlay for editing UI directly in the browser.
 * Uses Shadow DOM for complete isolation from user's app.
 */

// =============================================================================
// Type Definitions (JSDoc)
// =============================================================================

/**
 * @typedef {Object} PropMeta
 * @property {string} name
 * @property {string} ty
 * @property {boolean} required
 * @property {string|null} default
 * @property {string|null} doc
 */

/**
 * @typedef {Object} ComponentMeta
 * @property {string} name
 * @property {string} file
 * @property {number} line
 * @property {number} column
 * @property {string|null} doc
 * @property {PropMeta[]} props
 */

/**
 * @typedef {Object} Edit
 * @property {number} line
 * @property {string} old_text
 * @property {string} new_text
 */

/**
 * @typedef {Object} SelectedElement
 * @property {HTMLElement} element
 * @property {string|null} componentName
 * @property {string|null} sourceLocation
 */

// =============================================================================
// Studio State
// =============================================================================

const StudioState = {
  /** @type {boolean} */
  panelOpen: false,
  
  /** @type {boolean} */
  selectMode: false,
  
  /** @type {SelectedElement|null} */
  selectedElement: null,
  
  /** @type {ComponentMeta[]} */
  components: [],
  
  /** @type {WebSocket|null} */
  ws: null,
  
  /** @type {string} */
  activeTab: 'properties', // 'properties' | 'tree' | 'components'
};

// =============================================================================
// WebSocket Connection
// =============================================================================

function connectWebSocket() {
  const ws = new WebSocket('ws://localhost:3001/__studio');
  
  ws.onopen = () => {
    console.log('[Studio] Connected to dev server');
  };
  
  ws.onmessage = (event) => {
    try {
      const msg = JSON.parse(event.data);
      handleServerMessage(msg);
    } catch (e) {
      console.error('[Studio] Failed to parse message:', e);
    }
  };
  
  ws.onclose = () => {
    console.log('[Studio] Disconnected, reconnecting in 2s...');
    setTimeout(connectWebSocket, 2000);
  };
  
  ws.onerror = (error) => {
    console.error('[Studio] WebSocket error:', error);
  };
  
  StudioState.ws = ws;
}

/**
 * @param {Object} msg
 */
function handleServerMessage(msg) {
  switch (msg.type) {
    case 'edit_result':
      if (!msg.success) {
        console.error('[Studio] Edit failed:', msg.error);
        showNotification('Edit failed: ' + msg.error, 'error');
      } else {
        showNotification('Edit applied', 'success');
      }
      break;
    case 'file_updated':
      console.log('[Studio] File updated:', msg.file);
      break;
    case 'pong':
      // Keep-alive response
      break;
    case 'error':
      console.error('[Studio] Server error:', msg.message);
      showNotification(msg.message, 'error');
      break;
  }
}

/**
 * Send a message to the dev server
 * @param {Object} msg
 */
function sendMessage(msg) {
  if (StudioState.ws && StudioState.ws.readyState === WebSocket.OPEN) {
    StudioState.ws.send(JSON.stringify(msg));
  }
}

// =============================================================================
// Component Registry
// =============================================================================

async function fetchComponentRegistry() {
  try {
    const response = await fetch('/__studio/registry');
    const data = await response.json();
    StudioState.components = data.components || [];
    console.log('[Studio] Loaded', StudioState.components.length, 'components');
    
    // Update components panel if open
    const panel = document.querySelector('rejoice-studio');
    if (panel && panel.shadowRoot) {
      const componentsView = panel.shadowRoot.querySelector('studio-components');
      if (componentsView) {
        componentsView.render();
      }
    }
  } catch (e) {
    console.error('[Studio] Failed to fetch component registry:', e);
  }
}

// =============================================================================
// Notifications
// =============================================================================

/**
 * @param {string} message
 * @param {'success'|'error'|'info'} type
 */
function showNotification(message, type = 'info') {
  const studio = document.querySelector('rejoice-studio');
  if (studio && studio.shadowRoot) {
    const notification = document.createElement('div');
    notification.className = `studio-notification studio-notification-${type}`;
    notification.textContent = message;
    studio.shadowRoot.appendChild(notification);
    
    setTimeout(() => {
      notification.classList.add('studio-notification-fade');
      setTimeout(() => notification.remove(), 300);
    }, 2000);
  }
}

// =============================================================================
// Element Selection
// =============================================================================

/** @type {HTMLElement|null} */
let highlightOverlay = null;

function createHighlightOverlay() {
  if (highlightOverlay) return highlightOverlay;
  
  highlightOverlay = document.createElement('div');
  highlightOverlay.id = 'studio-highlight-overlay';
  highlightOverlay.style.cssText = `
    position: fixed;
    pointer-events: none;
    border: 2px solid #0066ff;
    background: rgba(0, 102, 255, 0.1);
    z-index: 999998;
    display: none;
  `;
  document.body.appendChild(highlightOverlay);
  return highlightOverlay;
}

/**
 * @param {HTMLElement} element
 */
function highlightElement(element) {
  const overlay = createHighlightOverlay();
  const rect = element.getBoundingClientRect();
  
  overlay.style.left = rect.left + 'px';
  overlay.style.top = rect.top + 'px';
  overlay.style.width = rect.width + 'px';
  overlay.style.height = rect.height + 'px';
  overlay.style.display = 'block';
}

function clearHighlight() {
  if (highlightOverlay) {
    highlightOverlay.style.display = 'none';
  }
}

/**
 * @param {HTMLElement} element
 * @returns {SelectedElement}
 */
function getElementInfo(element) {
  // Walk up to find component boundary
  let current = element;
  let componentName = null;
  let sourceLocation = null;
  
  while (current && current !== document.body) {
    if (current.dataset.component) {
      componentName = current.dataset.component;
    }
    if (current.dataset.source) {
      sourceLocation = current.dataset.source;
    }
    if (componentName && sourceLocation) break;
    current = current.parentElement;
  }
  
  return {
    element,
    componentName,
    sourceLocation,
  };
}

/**
 * Handle mouse move during select mode
 * @param {MouseEvent} event
 */
function handleSelectMouseMove(event) {
  if (!StudioState.selectMode) return;
  
  // Ignore studio elements
  const path = event.composedPath();
  if (path.some(el => el.tagName && el.tagName.toLowerCase().startsWith('studio-'))) {
    clearHighlight();
    return;
  }
  
  const target = /** @type {HTMLElement} */ (event.target);
  if (target && target !== document.body && target !== document.documentElement) {
    highlightElement(target);
  }
}

/**
 * Handle click during select mode
 * @param {MouseEvent} event
 */
function handleSelectClick(event) {
  if (!StudioState.selectMode) return;
  
  // Ignore studio elements
  const path = event.composedPath();
  if (path.some(el => el.tagName && el.tagName.toLowerCase().startsWith('studio-'))) {
    return;
  }
  
  event.preventDefault();
  event.stopPropagation();
  
  const target = /** @type {HTMLElement} */ (event.target);
  if (target && target !== document.body && target !== document.documentElement) {
    StudioState.selectedElement = getElementInfo(target);
    StudioState.selectMode = false;
    clearHighlight();
    
    // Update UI
    const studio = document.querySelector('rejoice-studio');
    if (studio) {
      studio.updateSelection();
    }
  }
}

// =============================================================================
// Main Studio Component
// =============================================================================

class RejoiceStudio extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }
  
  connectedCallback() {
    this.render();
    this.setupEventListeners();
    
    // Connect to dev server and fetch registry
    connectWebSocket();
    fetchComponentRegistry();
  }
  
  render() {
    this.shadowRoot.innerHTML = `
      <style>${STUDIO_STYLES}</style>
      <studio-toggle></studio-toggle>
      <studio-panel></studio-panel>
    `;
  }
  
  setupEventListeners() {
    // Global keyboard shortcut: Cmd/Ctrl + .
    document.addEventListener('keydown', (e) => {
      if ((e.metaKey || e.ctrlKey) && e.key === '.') {
        e.preventDefault();
        this.togglePanel();
      }
      // Escape to exit select mode or close panel
      if (e.key === 'Escape') {
        if (StudioState.selectMode) {
          StudioState.selectMode = false;
          clearHighlight();
          this.updateSelectMode();
        } else if (StudioState.panelOpen) {
          this.togglePanel();
        }
      }
    });
    
    // Select mode event listeners
    document.addEventListener('mousemove', handleSelectMouseMove, true);
    document.addEventListener('click', handleSelectClick, true);
  }
  
  togglePanel() {
    StudioState.panelOpen = !StudioState.panelOpen;
    const panel = this.shadowRoot.querySelector('studio-panel');
    if (panel) {
      panel.classList.toggle('open', StudioState.panelOpen);
    }
    const toggle = this.shadowRoot.querySelector('studio-toggle');
    if (toggle) {
      toggle.classList.toggle('panel-open', StudioState.panelOpen);
    }
    
    // Push page content over instead of overlapping
    document.documentElement.style.transition = 'margin-right 0.3s ease';
    document.documentElement.style.marginRight = StudioState.panelOpen ? '400px' : '';
    
    // Also adjust fixed/sticky elements that won't respond to margin
    adjustFixedElements(StudioState.panelOpen);
  }
  
  updateSelection() {
    const panel = this.shadowRoot.querySelector('studio-panel');
    if (panel) {
      panel.updateSelection();
    }
  }
  
  updateSelectMode() {
    const panel = this.shadowRoot.querySelector('studio-panel');
    if (panel) {
      panel.updateSelectMode();
    }
  }
}

// =============================================================================
// Toggle Button Component
// =============================================================================

class StudioToggle extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }
  
  connectedCallback() {
    this.render();
    this.shadowRoot.querySelector('button').addEventListener('click', () => {
      // Use getRootNode() to traverse Shadow DOM boundary
      const rootNode = this.getRootNode();
      const studio = rootNode.host;
      if (studio && studio.tagName === 'REJOICE-STUDIO') {
        studio.togglePanel();
      }
    });
  }
  
  render() {
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          position: fixed;
          bottom: 20px;
          right: 20px;
          z-index: 999999;
          transition: right 0.3s ease;
        }
        :host(.panel-open) {
          right: 420px;
        }
        button {
          width: 48px;
          height: 48px;
          border-radius: 50%;
          border: none;
          background: #1e1e1e;
          color: white;
          cursor: pointer;
          display: flex;
          align-items: center;
          justify-content: center;
          box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
          transition: transform 0.2s, background 0.2s;
        }
        button:hover {
          background: #2d2d2d;
          transform: scale(1.05);
        }
        :host(.panel-open) button {
          background: #0066ff;
        }
        svg {
          width: 24px;
          height: 24px;
        }
      </style>
      <button title="Toggle Studio (Cmd+.)">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2L2 7l10 5 10-5-10-5z"/>
          <path d="M2 17l10 5 10-5"/>
          <path d="M2 12l10 5 10-5"/>
        </svg>
      </button>
    `;
  }
}

// =============================================================================
// Main Panel Component
// =============================================================================

class StudioPanel extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }
  
  connectedCallback() {
    this.render();
    this.setupEventListeners();
  }
  
  render() {
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          position: fixed;
          top: 0;
          right: -400px;
          width: 400px;
          height: 100vh;
          background: #1e1e1e;
          color: #e0e0e0;
          font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
          font-size: 13px;
          box-shadow: -4px 0 20px rgba(0, 0, 0, 0.3);
          transition: right 0.3s ease;
          display: flex;
          flex-direction: column;
          z-index: 999999;
        }
        :host(.open) {
          right: 0;
        }
        .toolbar {
          display: flex;
          gap: 4px;
          padding: 12px;
          border-bottom: 1px solid #333;
          background: #252525;
        }
        .toolbar button {
          padding: 8px 12px;
          border: none;
          background: transparent;
          color: #999;
          cursor: pointer;
          border-radius: 4px;
          font-size: 12px;
          transition: background 0.2s, color 0.2s;
        }
        .toolbar button:hover {
          background: #333;
          color: #fff;
        }
        .toolbar button.active {
          background: #0066ff;
          color: #fff;
        }
        .toolbar button.select-mode {
          background: ${StudioState.selectMode ? '#0066ff' : 'transparent'};
          color: ${StudioState.selectMode ? '#fff' : '#999'};
        }
        .content {
          flex: 1;
          overflow-y: auto;
          padding: 16px;
        }
        .tab-content {
          display: none;
        }
        .tab-content.active {
          display: block;
        }
      </style>
      <div class="toolbar">
        <button class="select-btn ${StudioState.selectMode ? 'active' : ''}" title="Select Element">
          <span>Select</span>
        </button>
        <button class="tab-btn ${StudioState.activeTab === 'properties' ? 'active' : ''}" data-tab="properties">
          Properties
        </button>
        <button class="tab-btn ${StudioState.activeTab === 'tree' ? 'active' : ''}" data-tab="tree">
          Tree
        </button>
        <button class="tab-btn ${StudioState.activeTab === 'components' ? 'active' : ''}" data-tab="components">
          Components
        </button>
      </div>
      <div class="content">
        <div class="tab-content ${StudioState.activeTab === 'properties' ? 'active' : ''}" data-tab="properties">
          <studio-properties></studio-properties>
        </div>
        <div class="tab-content ${StudioState.activeTab === 'tree' ? 'active' : ''}" data-tab="tree">
          <studio-tree></studio-tree>
        </div>
        <div class="tab-content ${StudioState.activeTab === 'components' ? 'active' : ''}" data-tab="components">
          <studio-components></studio-components>
        </div>
      </div>
    `;
  }
  
  setupEventListeners() {
    // Select button
    this.shadowRoot.querySelector('.select-btn').addEventListener('click', () => {
      StudioState.selectMode = !StudioState.selectMode;
      if (!StudioState.selectMode) {
        clearHighlight();
      }
      this.updateSelectMode();
    });
    
    // Tab buttons
    this.shadowRoot.querySelectorAll('.tab-btn').forEach(btn => {
      btn.addEventListener('click', (e) => {
        const tab = e.target.dataset.tab;
        StudioState.activeTab = tab;
        this.updateTabs();
      });
    });
  }
  
  updateSelectMode() {
    const btn = this.shadowRoot.querySelector('.select-btn');
    btn.classList.toggle('active', StudioState.selectMode);
  }
  
  updateTabs() {
    this.shadowRoot.querySelectorAll('.tab-btn').forEach(btn => {
      btn.classList.toggle('active', btn.dataset.tab === StudioState.activeTab);
    });
    this.shadowRoot.querySelectorAll('.tab-content').forEach(content => {
      content.classList.toggle('active', content.dataset.tab === StudioState.activeTab);
    });
  }
  
  updateSelection() {
    const properties = this.shadowRoot.querySelector('studio-properties');
    if (properties) {
      properties.render();
    }
    this.updateSelectMode();
  }
}

// =============================================================================
// Properties Panel Component
// =============================================================================

class StudioProperties extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }
  
  connectedCallback() {
    this.render();
  }
  
  render() {
    const selected = StudioState.selectedElement;
    
    if (!selected) {
      this.shadowRoot.innerHTML = `
        <style>${PROPERTIES_STYLES}</style>
        <div class="empty-state">
          <p>No element selected</p>
          <p class="hint">Click "Select" and then click an element on the page</p>
        </div>
      `;
      return;
    }
    
    const element = selected.element;
    const classes = element.className || '';
    const tagName = element.tagName.toLowerCase();
    
    this.shadowRoot.innerHTML = `
      <style>${PROPERTIES_STYLES}</style>
      
      <div class="section">
        <div class="section-header">Element</div>
        <div class="info-row">
          <span class="label">Tag:</span>
          <span class="value">&lt;${tagName}&gt;</span>
        </div>
        ${selected.componentName ? `
          <div class="info-row">
            <span class="label">Component:</span>
            <span class="value component-name">${selected.componentName}</span>
          </div>
        ` : ''}
        ${selected.sourceLocation ? `
          <div class="info-row">
            <span class="label">Source:</span>
            <span class="value source-link">${selected.sourceLocation}</span>
          </div>
        ` : ''}
      </div>
      
      <div class="section">
        <div class="section-header">Classes</div>
        <textarea class="classes-input" placeholder="Enter classes...">${classes}</textarea>
        <button class="apply-btn">Apply Changes</button>
      </div>
      
      ${element.id ? `
        <div class="section">
          <div class="section-header">ID</div>
          <div class="info-row">
            <span class="value">#${element.id}</span>
          </div>
        </div>
      ` : ''}
    `;
    
    // Setup event listeners
    const textarea = this.shadowRoot.querySelector('.classes-input');
    const applyBtn = this.shadowRoot.querySelector('.apply-btn');
    
    if (applyBtn && textarea) {
      applyBtn.addEventListener('click', () => {
        this.applyClassChanges(textarea.value);
      });
    }
  }
  
  /**
   * @param {string} newClasses
   */
  applyClassChanges(newClasses) {
    const selected = StudioState.selectedElement;
    if (!selected || !selected.sourceLocation) {
      showNotification('Cannot edit: no source location', 'error');
      return;
    }
    
    const oldClasses = selected.element.className || '';
    if (oldClasses === newClasses) return;
    
    // Parse source location (file:line:col)
    const [file, lineStr] = selected.sourceLocation.split(':');
    const line = parseInt(lineStr, 10);
    
    if (!file || isNaN(line)) {
      showNotification('Invalid source location', 'error');
      return;
    }
    
    // Apply optimistically to DOM
    selected.element.className = newClasses;
    
    // Send edit to server
    // This is a simple implementation - finding class="..." on the line
    const oldText = `class="${oldClasses}"`;
    const newText = `class="${newClasses}"`;
    
    sendMessage({
      type: 'edit_file',
      file: file,
      edits: [{
        line: line,
        old_text: oldText,
        new_text: newText,
      }]
    });
  }
}

// =============================================================================
// Tree View Component
// =============================================================================

class StudioTree extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }
  
  connectedCallback() {
    this.render();
  }
  
  render() {
    this.shadowRoot.innerHTML = `
      <style>${TREE_STYLES}</style>
      <div class="tree-container">
        ${this.renderNode(document.body, 0)}
      </div>
    `;
    
    // Setup click handlers
    this.shadowRoot.querySelectorAll('.tree-node').forEach(node => {
      node.addEventListener('click', (e) => {
        e.stopPropagation();
        const path = node.dataset.path;
        const element = this.getElementByPath(path);
        if (element) {
          StudioState.selectedElement = getElementInfo(element);
          const studio = document.querySelector('rejoice-studio');
          if (studio) {
            studio.updateSelection();
          }
        }
      });
    });
  }
  
  /**
   * @param {HTMLElement} element
   * @param {number} depth
   * @param {string} path
   * @returns {string}
   */
  renderNode(element, depth, path = '0') {
    if (depth > 10) return ''; // Limit depth
    if (!element || !element.tagName) return '';
    
    // Skip studio elements and scripts
    const tagName = element.tagName.toLowerCase();
    if (tagName.startsWith('studio-') || tagName === 'rejoice-studio' || 
        tagName === 'script' || tagName === 'style') {
      return '';
    }
    
    const componentName = element.dataset?.component;
    const hasChildren = element.children.length > 0;
    const indent = depth * 16;
    
    let childrenHtml = '';
    if (hasChildren) {
      for (let i = 0; i < element.children.length; i++) {
        childrenHtml += this.renderNode(element.children[i], depth + 1, `${path}-${i}`);
      }
    }
    
    return `
      <div class="tree-node" style="padding-left: ${indent}px" data-path="${path}">
        <span class="tag">&lt;${tagName}&gt;</span>
        ${componentName ? `<span class="component">${componentName}</span>` : ''}
      </div>
      ${childrenHtml}
    `;
  }
  
  /**
   * @param {string} path
   * @returns {HTMLElement|null}
   */
  getElementByPath(path) {
    const indices = path.split('-').map(Number);
    let element = document.body;
    
    for (let i = 1; i < indices.length; i++) {
      if (!element.children[indices[i]]) return null;
      element = element.children[indices[i]];
    }
    
    return element;
  }
}

// =============================================================================
// Components Browser Component
// =============================================================================

class StudioComponents extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }
  
  connectedCallback() {
    this.render();
  }
  
  render() {
    const components = StudioState.components;
    
    if (components.length === 0) {
      this.shadowRoot.innerHTML = `
        <style>${COMPONENTS_STYLES}</style>
        <div class="empty-state">
          <p>No components registered</p>
          <p class="hint">Components will appear here after they render</p>
        </div>
      `;
      return;
    }
    
    this.shadowRoot.innerHTML = `
      <style>${COMPONENTS_STYLES}</style>
      <div class="components-list">
        ${components.map(c => this.renderComponent(c)).join('')}
      </div>
    `;
  }
  
  /**
   * @param {ComponentMeta} component
   * @returns {string}
   */
  renderComponent(component) {
    const props = component.props || [];
    
    return `
      <div class="component-card">
        <div class="component-header">
          <span class="component-name">${component.name}</span>
          <span class="component-source">${component.file}:${component.line}</span>
        </div>
        ${component.doc ? `<div class="component-doc">${component.doc}</div>` : ''}
        ${props.length > 0 ? `
          <div class="props-list">
            <div class="props-header">Props</div>
            ${props.map(p => `
              <div class="prop-item">
                <span class="prop-name">${p.name}${p.required ? '*' : ''}</span>
                <span class="prop-type">${p.ty}</span>
                ${p.default ? `<span class="prop-default">= ${p.default}</span>` : ''}
              </div>
            `).join('')}
          </div>
        ` : ''}
      </div>
    `;
  }
}

// =============================================================================
// Styles
// =============================================================================

const STUDIO_STYLES = `
  :host {
    all: initial;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }
  
  .studio-notification {
    position: fixed;
    bottom: 80px;
    right: 20px;
    padding: 12px 20px;
    border-radius: 8px;
    color: white;
    font-size: 13px;
    z-index: 1000000;
    animation: slideIn 0.3s ease;
  }
  
  .studio-notification-success {
    background: #10b981;
  }
  
  .studio-notification-error {
    background: #ef4444;
  }
  
  .studio-notification-info {
    background: #3b82f6;
  }
  
  .studio-notification-fade {
    opacity: 0;
    transition: opacity 0.3s;
  }
  
  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
`;

const PROPERTIES_STYLES = `
  :host {
    display: block;
  }
  
  .empty-state {
    text-align: center;
    padding: 40px 20px;
    color: #666;
  }
  
  .empty-state .hint {
    font-size: 12px;
    margin-top: 8px;
    color: #555;
  }
  
  .section {
    margin-bottom: 20px;
  }
  
  .section-header {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    color: #888;
    margin-bottom: 8px;
    letter-spacing: 0.5px;
  }
  
  .info-row {
    display: flex;
    justify-content: space-between;
    padding: 6px 0;
    border-bottom: 1px solid #333;
  }
  
  .label {
    color: #888;
  }
  
  .value {
    color: #e0e0e0;
  }
  
  .component-name {
    color: #66d9ef;
  }
  
  .source-link {
    color: #a6e22e;
    font-family: monospace;
    font-size: 12px;
  }
  
  .classes-input {
    width: 100%;
    min-height: 80px;
    padding: 10px;
    border: 1px solid #333;
    border-radius: 4px;
    background: #252525;
    color: #e0e0e0;
    font-family: monospace;
    font-size: 12px;
    resize: vertical;
    box-sizing: border-box;
  }
  
  .classes-input:focus {
    outline: none;
    border-color: #0066ff;
  }
  
  .apply-btn {
    margin-top: 8px;
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    background: #0066ff;
    color: white;
    cursor: pointer;
    font-size: 13px;
    transition: background 0.2s;
  }
  
  .apply-btn:hover {
    background: #0055dd;
  }
`;

const TREE_STYLES = `
  :host {
    display: block;
  }
  
  .tree-container {
    font-family: monospace;
    font-size: 12px;
  }
  
  .tree-node {
    padding: 4px 8px;
    cursor: pointer;
    border-radius: 4px;
    white-space: nowrap;
  }
  
  .tree-node:hover {
    background: #333;
  }
  
  .tag {
    color: #66d9ef;
  }
  
  .component {
    margin-left: 8px;
    padding: 2px 6px;
    background: #3d3d3d;
    border-radius: 3px;
    color: #a6e22e;
    font-size: 10px;
  }
`;

const COMPONENTS_STYLES = `
  :host {
    display: block;
  }
  
  .empty-state {
    text-align: center;
    padding: 40px 20px;
    color: #666;
  }
  
  .empty-state .hint {
    font-size: 12px;
    margin-top: 8px;
    color: #555;
  }
  
  .components-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  
  .component-card {
    background: #252525;
    border-radius: 8px;
    padding: 12px;
  }
  
  .component-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }
  
  .component-name {
    font-weight: 600;
    color: #66d9ef;
  }
  
  .component-source {
    font-size: 11px;
    color: #666;
    font-family: monospace;
  }
  
  .component-doc {
    font-size: 12px;
    color: #999;
    margin-bottom: 12px;
    font-style: italic;
  }
  
  .props-list {
    border-top: 1px solid #333;
    padding-top: 8px;
  }
  
  .props-header {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    color: #666;
    margin-bottom: 6px;
  }
  
  .prop-item {
    display: flex;
    gap: 8px;
    padding: 4px 0;
    font-size: 12px;
    font-family: monospace;
  }
  
  .prop-name {
    color: #f8f8f2;
  }
  
  .prop-type {
    color: #66d9ef;
  }
  
  .prop-default {
    color: #888;
  }
`;

// =============================================================================
// Register Web Components
// =============================================================================

customElements.define('rejoice-studio', RejoiceStudio);
customElements.define('studio-toggle', StudioToggle);
customElements.define('studio-panel', StudioPanel);
customElements.define('studio-properties', StudioProperties);
customElements.define('studio-tree', StudioTree);
customElements.define('studio-components', StudioComponents);

// =============================================================================
// Initialize
// =============================================================================

// Add the studio element to the page
if (!document.querySelector('rejoice-studio')) {
  document.body.appendChild(document.createElement('rejoice-studio'));
}
