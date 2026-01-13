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
      post({ 
        type: 'hover', 
        rect: { left: r.left, top: r.top, width: r.width, height: r.height },
        tagName: el.tagName.toLowerCase()
      });
    }
  }
  else if (m.type === 'hover-end-tree') {
    post({ type: 'hover-end' });
  }
});

const post = m => window.parent.postMessage(m, '*');

// Hover
document.addEventListener('mousemove', e => {
  if (!selectMode) return;
  const el = validEl(e.target);
  if (!el || el === lastHover) return;
  lastHover = el;
  const r = el.getBoundingClientRect();
  post({ 
    type: 'hover', 
    rect: { left: r.left, top: r.top, width: r.width, height: r.height },
    tagName: el.tagName.toLowerCase()
  });
}, true);

document.addEventListener('mouseleave', () => {
  if (selectMode) { lastHover = null; post({ type: 'hover-end' }); }
}, true);

// Click
document.addEventListener('click', e => {
  if (!selectMode) return;
  e.preventDefault();
  e.stopPropagation();
  const el = validEl(e.target);
  if (el) selectEl(el);
}, true);

function validEl(el) {
  if (!el || el === document.body || el === document.documentElement) return null;
  const t = el.tagName?.toLowerCase();
  if (!t || t === 'script' || t === 'style' || t === 'link' || t === 'meta') return null;
  return el;
}

function selectEl(el) {
  let comp = null, src = null, cur = el;
  while (cur && cur !== document.body) {
    if (cur.dataset?.component) comp = cur.dataset.component;
    if (cur.dataset?.source) src = cur.dataset.source;
    if (comp && src) break;
    cur = cur.parentElement;
  }
  post({
    type: 'selected',
    tagName: el.tagName.toLowerCase(),
    classes: el.className || '',
    id: el.id || null,
    componentName: comp,
    sourceLocation: src,
    path: getPath(el),
  });
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
`;
document.head.appendChild(s);
