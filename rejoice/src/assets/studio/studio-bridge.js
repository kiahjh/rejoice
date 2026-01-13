/**
 * Rejoice Studio Bridge
 * Handles element selection and DOM inspection inside the iframe.
 */

// Inject Tailwind CDN for instant class support in Studio
(function injectTailwindCDN() {
  const script = document.createElement('script');
  script.src = 'https://cdn.tailwindcss.com';
  document.head.appendChild(script);
})();

let selectMode = false;
let lastHover = null;

// Messages from host
window.addEventListener('message', e => {
  const m = e.data;
  if (!m || typeof m !== 'object') return;
  
  if (m.type === 'set-select-mode') {
    selectMode = m.enabled;
    document.documentElement.classList.toggle('studio-select', m.enabled);
    if (!m.enabled) lastHover = null;
  }
  else if (m.type === 'preview-classes') {
    // Instant preview - just update classes, no highlight flash
    const el = getByPath(m.path);
    if (el) {
      el.className = m.classes;
    }
  }
  else if (m.type === 'get-tree') {
    post({ type: 'tree-data', tree: buildTree(document.body, '0', 0) });
  }
  else if (m.type === 'select-by-path') {
    const el = getByPath(m.path);
    if (el) {
      selectEl(el);
      el.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }
  }
  else if (m.type === 'hover-by-path') {
    const el = getByPath(m.path);
    if (el) {
      const r = el.getBoundingClientRect();
      const compInfo = getComponentInfo(el);
      const isComponentRoot = !!el.dataset?.component || !!el.parentElement?.dataset?.component;
      post({ 
        type: 'hover', 
        rect: { left: r.left, top: r.top, width: r.width, height: r.height },
        tagName: el.tagName.toLowerCase(),
        componentName: compInfo?.name || null,
        isComponentRoot
      });
    }
  }
  else if (m.type === 'hover-end-tree') {
    post({ type: 'hover-end' });
  }
  else if (m.type === 'get-components-on-page') {
    // Find all elements with data-component attribute
    const elements = document.querySelectorAll('[data-component]');
    const components = {};
    elements.forEach(el => {
      const name = el.dataset.component;
      if (!components[name]) {
        components[name] = [];
      }
      components[name].push(getPath(el));
    });
    post({ type: 'components-on-page', components });
  }
  else if (m.type === 'reveal-component') {
    // Find first instance of component and scroll to it
    const el = document.querySelector(`[data-component="${m.name}"]`);
    if (el) {
      el.scrollIntoView({ behavior: 'smooth', block: 'center' });
      // Brief highlight effect
      el.style.outline = '3px solid #f0abfc';
      el.style.outlineOffset = '2px';
      el.style.transition = 'outline-color 0.3s ease';
      setTimeout(() => {
        el.style.outlineColor = 'transparent';
        setTimeout(() => {
          el.style.outline = '';
          el.style.outlineOffset = '';
          el.style.transition = '';
        }, 300);
      }, 800);
      // Also select it
      selectEl(el);
    }
  }
});

const post = m => window.parent.postMessage(m, '*');

function validEl(el) {
  if (!el || el === document.body || el === document.documentElement) return null;
  if (el.id === 'studio-select-overlay') return null;
  const t = el.tagName?.toLowerCase();
  if (!t || t === 'script' || t === 'style' || t === 'link' || t === 'meta') return null;
  
  // For SVG child elements (path, rect, circle, etc.), walk up to the <svg> element
  // or to the first HTML element parent, whichever comes first
  if (el instanceof SVGElement && t !== 'svg') {
    let cur = el.parentElement;
    while (cur && cur !== document.body) {
      if (cur.tagName.toLowerCase() === 'svg' || !(cur instanceof SVGElement)) {
        return cur;
      }
      cur = cur.parentElement;
    }
    return null;
  }
  
  return el;
}

// Helper to get dataset property, works for both HTML and SVG elements
function getDataAttr(el, name) {
  if (!el) return null;
  // HTML elements have dataset
  if (el.dataset) return el.dataset[name] || null;
  // SVG elements need getAttribute
  return el.getAttribute?.(`data-${name}`) || null;
}

function selectEl(el) {
  let comp = null, src = null, compRoot = null, cur = el;
  while (cur && cur !== document.body) {
    const compAttr = getDataAttr(cur, 'component');
    if (compAttr) {
      comp = compAttr;
      compRoot = cur; // The element that has data-component
    }
    const srcAttr = getDataAttr(cur, 'source');
    if (srcAttr) src = srcAttr;
    if (comp && src) break;
    cur = cur.parentElement;
  }
  // Check if the selected element is the component root itself
  // The macro wraps component content in a div with data-component, so the "root"
  // from the user's perspective is actually the first child of that wrapper.
  // We consider it "root" if: the element itself has data-component, OR its parent does.
  const isComponentRoot = !!getDataAttr(el, 'component') || !!getDataAttr(el.parentElement, 'component');
  
  // Get className safely (SVG elements have className as SVGAnimatedString)
  let classes = '';
  if (typeof el.className === 'string') {
    classes = el.className;
  } else if (el.className?.baseVal) {
    classes = el.className.baseVal;
  } else if (el.getAttribute) {
    classes = el.getAttribute('class') || '';
  }
  
  post({
    type: 'selected',
    tagName: el.tagName.toLowerCase(),
    classes,
    id: el.id || null,
    componentName: comp,
    isComponentRoot,
    sourceLocation: src,
    path: getPath(el),
  });
}

// Get component info for an element (checks if element or ancestor is a component)
function getComponentInfo(el) {
  let cur = el;
  while (cur && cur !== document.body) {
    if (cur.dataset?.component) {
      return { name: cur.dataset.component, element: cur };
    }
    cur = cur.parentElement;
  }
  return null;
}

// Path utilities
function getPath(el) {
  const idx = [];
  let cur = el;
  while (cur && cur !== document.body) {
    const p = cur.parentElement;
    if (p) idx.unshift(Array.from(p.children).indexOf(cur));
    cur = p;
  }
  return '0-' + idx.join('-');
}

function getByPath(path) {
  const idx = path.split('-').map(Number);
  let el = document.body;
  for (let i = 1; i < idx.length && el; i++) el = el.children[idx[i]];
  return el || null;
}

// Tree
function buildTree(el, path, depth) {
  if (!el?.tagName || depth > 12) return null;
  const t = el.tagName.toLowerCase();
  if (t === 'script' || t === 'style' || t === 'svg') return null; // Skip SVGs for now
  if (el.id === 'studio-select-overlay') return null; // Skip studio overlay
  
  // Get className safely (SVG elements have className as SVGAnimatedString)
  let classStr = '';
  if (typeof el.className === 'string') {
    classStr = el.className;
  } else if (el.className?.baseVal) {
    classStr = el.className.baseVal;
  } else if (el.getAttribute) {
    classStr = el.getAttribute('class') || '';
  }
  
  const node = { 
    tagName: t, 
    path, 
    componentName: el.dataset?.component || null,
    id: el.id || null,
    classes: classStr ? classStr.split(/\s+/).filter(Boolean).slice(0, 3) : [],
    children: [] 
  };
  for (let i = 0; i < el.children.length; i++) {
    const c = buildTree(el.children[i], `${path}-${i}`, depth + 1);
    if (c) node.children.push(c);
  }
  return node;
}

// Nav sync
const pushOrig = history.pushState;
history.pushState = function(...a) { pushOrig.apply(this, a); post({ type: 'navigate', path: location.pathname + location.search }); };
const replOrig = history.replaceState;
history.replaceState = function(...a) { replOrig.apply(this, a); post({ type: 'navigate', path: location.pathname + location.search }); };
window.addEventListener('popstate', () => post({ type: 'navigate', path: location.pathname + location.search }));

// Intercept link clicks to preserve __studio_bridge param
document.addEventListener('click', e => {
  const link = e.target.closest('a[href]');
  if (!link) return;
  
  const href = link.getAttribute('href');
  // Only handle internal links (not external, not anchor-only, not javascript:)
  if (!href || href.startsWith('http') || href.startsWith('#') || href.startsWith('javascript:')) return;
  
  // Add __studio_bridge param to the URL
  e.preventDefault();
  const url = new URL(href, location.origin);
  url.searchParams.set('__studio_bridge', '1');
  location.href = url.toString();
}, true);

// Forward keyboard shortcuts to host
document.addEventListener('keydown', e => {
  // Cmd/Ctrl + . to toggle panel
  if ((e.metaKey || e.ctrlKey) && e.key === '.') {
    e.preventDefault();
    post({ type: 'shortcut', key: 'toggle' });
  }
  // Escape
  if (e.key === 'Escape') {
    post({ type: 'shortcut', key: 'escape' });
  }
  // S to select (when not in input)
  if (e.key === 's' && !e.target.matches('input,textarea')) {
    e.preventDefault();
    post({ type: 'shortcut', key: 'select' });
  }
});

// Style
const s = document.createElement('style');
s.textContent = `
  html.studio-select, html.studio-select * { 
    cursor: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z" fill="%23f0abfc" stroke="%23ffffff" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>') 3 3, crosshair !important; 
  }
  #studio-select-overlay {
    position: fixed;
    inset: 0;
    z-index: 999999;
    display: none;
    cursor: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z" fill="%23f0abfc" stroke="%23ffffff" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>') 3 3, crosshair !important;
  }
  html.studio-select #studio-select-overlay {
    display: block;
  }
`;
document.head.appendChild(s);

// Create overlay for select mode (captures all pointer events, prevents hover states)
const overlay = document.createElement('div');
overlay.id = 'studio-select-overlay';
document.body.appendChild(overlay);

// Handle mouse events on overlay
overlay.addEventListener('mousemove', e => {
  // Find element under cursor (temporarily hide overlay)
  overlay.style.display = 'none';
  const el = document.elementFromPoint(e.clientX, e.clientY);
  overlay.style.display = '';
  
  const target = validEl(el);
  if (!target || target === lastHover) return;
  lastHover = target;
  const r = target.getBoundingClientRect();
  const compInfo = getComponentInfo(target);
  // Check if this element is the component root (has data-component or parent does)
  const isComponentRoot = !!target.dataset?.component || !!target.parentElement?.dataset?.component;
  post({ 
    type: 'hover', 
    rect: { left: r.left, top: r.top, width: r.width, height: r.height },
    tagName: target.tagName.toLowerCase(),
    componentName: compInfo?.name || null,
    isComponentRoot
  });
});

overlay.addEventListener('click', e => {
  // Find element under cursor
  overlay.style.display = 'none';
  const el = document.elementFromPoint(e.clientX, e.clientY);
  overlay.style.display = '';
  
  const target = validEl(el);
  if (target) selectEl(target);
});

overlay.addEventListener('mouseleave', () => {
  lastHover = null;
  post({ type: 'hover-end' });
});

// Announce that bridge is ready
post({ type: 'bridge-ready' });
