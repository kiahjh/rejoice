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
  enumVariants: {}, // { typeName: ["Variant1", "Variant2", ...] }
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
  // Design tools state
  activeBreakpoint: "base", // "base", "sm", "md", "lg", "xl", "2xl"
  activeState: "normal", // "normal", "hover", "focus", "active"
};

const MIN_PANEL_WIDTH = 380;

// Tailwind breakpoints (in pixels)
const BREAKPOINTS = {
  base: null, // No max width - full canvas
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  "2xl": 1536,
};

// Tailwind spacing scale (maps to px values)
const SPACING_SCALE = {
  "0": 0,
  "px": 1,
  "0.5": 2,
  "1": 4,
  "1.5": 6,
  "2": 8,
  "2.5": 10,
  "3": 12,
  "3.5": 14,
  "4": 16,
  "5": 20,
  "6": 24,
  "7": 28,
  "8": 32,
  "9": 36,
  "10": 40,
  "11": 44,
  "12": 48,
  "14": 56,
  "16": 64,
  "20": 80,
  "24": 96,
  "28": 112,
  "32": 128,
  "36": 144,
  "40": 160,
  "44": 176,
  "48": 192,
  "52": 208,
  "56": 224,
  "60": 240,
  "64": 256,
  "72": 288,
  "80": 320,
  "96": 384,
};

// Reverse lookup: px to Tailwind value
const PX_TO_SPACING = Object.fromEntries(
  Object.entries(SPACING_SCALE).map(([k, v]) => [v, k])
);

// Ordered spacing values for stepper
const SPACING_VALUES = Object.keys(SPACING_SCALE);

// Size scale for width/height
const SIZE_SCALE = {
  "0": "0px",
  "px": "1px",
  "0.5": "0.125rem",
  "1": "0.25rem",
  "1.5": "0.375rem",
  "2": "0.5rem",
  "2.5": "0.625rem",
  "3": "0.75rem",
  "3.5": "0.875rem",
  "4": "1rem",
  "5": "1.25rem",
  "6": "1.5rem",
  "7": "1.75rem",
  "8": "2rem",
  "9": "2.25rem",
  "10": "2.5rem",
  "11": "2.75rem",
  "12": "3rem",
  "14": "3.5rem",
  "16": "4rem",
  "20": "5rem",
  "24": "6rem",
  "28": "7rem",
  "32": "8rem",
  "36": "9rem",
  "40": "10rem",
  "44": "11rem",
  "48": "12rem",
  "52": "13rem",
  "56": "14rem",
  "60": "15rem",
  "64": "16rem",
  "72": "18rem",
  "80": "20rem",
  "96": "24rem",
  // Fractional sizes
  "1/2": "50%",
  "1/3": "33.333%",
  "2/3": "66.667%",
  "1/4": "25%",
  "3/4": "75%",
  "full": "100%",
  "screen": "100vw",
  "min": "min-content",
  "max": "max-content",
  "fit": "fit-content",
  "auto": "auto",
};

// Font size scale
const FONT_SIZE_SCALE = {
  "xs": "0.75rem",
  "sm": "0.875rem",
  "base": "1rem",
  "lg": "1.125rem",
  "xl": "1.25rem",
  "2xl": "1.5rem",
  "3xl": "1.875rem",
  "4xl": "2.25rem",
  "5xl": "3rem",
  "6xl": "3.75rem",
  "7xl": "4.5rem",
  "8xl": "6rem",
  "9xl": "8rem",
};

const FONT_SIZE_VALUES = Object.keys(FONT_SIZE_SCALE);

// Font weight scale
const FONT_WEIGHT_SCALE = {
  "thin": "100",
  "extralight": "200",
  "light": "300",
  "normal": "400",
  "medium": "500",
  "semibold": "600",
  "bold": "700",
  "extrabold": "800",
  "black": "900",
};

// Border radius scale
const RADIUS_SCALE = {
  "none": "0px",
  "sm": "0.125rem",
  "": "0.25rem",
  "md": "0.375rem",
  "lg": "0.5rem",
  "xl": "0.75rem",
  "2xl": "1rem",
  "3xl": "1.5rem",
  "full": "9999px",
};

const RADIUS_VALUES = ["none", "sm", "", "md", "lg", "xl", "2xl", "3xl", "full"];

// Border width scale
const BORDER_WIDTH_SCALE = {
  "0": "0px",
  "": "1px",
  "2": "2px",
  "4": "4px",
  "8": "8px",
};

// Opacity scale
const OPACITY_SCALE = {
  "0": "0",
  "5": "0.05",
  "10": "0.1",
  "15": "0.15",
  "20": "0.2",
  "25": "0.25",
  "30": "0.3",
  "35": "0.35",
  "40": "0.4",
  "45": "0.45",
  "50": "0.5",
  "55": "0.55",
  "60": "0.6",
  "65": "0.65",
  "70": "0.7",
  "75": "0.75",
  "80": "0.8",
  "85": "0.85",
  "90": "0.9",
  "95": "0.95",
  "100": "1",
};

const OPACITY_VALUES = Object.keys(OPACITY_SCALE);

// Shadow scale
const SHADOW_SCALE = {
  "none": "none",
  "sm": "0 1px 2px 0 rgb(0 0 0 / 0.05)",
  "": "0 1px 3px 0 rgb(0 0 0 / 0.1)",
  "md": "0 4px 6px -1px rgb(0 0 0 / 0.1)",
  "lg": "0 10px 15px -3px rgb(0 0 0 / 0.1)",
  "xl": "0 20px 25px -5px rgb(0 0 0 / 0.1)",
  "2xl": "0 25px 50px -12px rgb(0 0 0 / 0.25)",
};

const SHADOW_VALUES = ["none", "sm", "", "md", "lg", "xl", "2xl"];

// Complete Tailwind v4 colors
const TAILWIND_COLORS = {
  "transparent": "transparent",
  "current": "currentColor",
  "black": "#000000",
  "white": "#ffffff",
  // Slate
  "slate-50": "#f8fafc", "slate-100": "#f1f5f9", "slate-200": "#e2e8f0", "slate-300": "#cbd5e1",
  "slate-400": "#94a3b8", "slate-500": "#64748b", "slate-600": "#475569", "slate-700": "#334155",
  "slate-800": "#1e293b", "slate-900": "#0f172a", "slate-950": "#020617",
  // Gray
  "gray-50": "#f9fafb", "gray-100": "#f3f4f6", "gray-200": "#e5e7eb", "gray-300": "#d1d5db",
  "gray-400": "#9ca3af", "gray-500": "#6b7280", "gray-600": "#4b5563", "gray-700": "#374151",
  "gray-800": "#1f2937", "gray-900": "#111827", "gray-950": "#030712",
  // Zinc
  "zinc-50": "#fafafa", "zinc-100": "#f4f4f5", "zinc-200": "#e4e4e7", "zinc-300": "#d4d4d8",
  "zinc-400": "#a1a1aa", "zinc-500": "#71717a", "zinc-600": "#52525b", "zinc-700": "#3f3f46",
  "zinc-800": "#27272a", "zinc-900": "#18181b", "zinc-950": "#09090b",
  // Neutral
  "neutral-50": "#fafafa", "neutral-100": "#f5f5f5", "neutral-200": "#e5e5e5", "neutral-300": "#d4d4d4",
  "neutral-400": "#a3a3a3", "neutral-500": "#737373", "neutral-600": "#525252", "neutral-700": "#404040",
  "neutral-800": "#262626", "neutral-900": "#171717", "neutral-950": "#0a0a0a",
  // Stone
  "stone-50": "#fafaf9", "stone-100": "#f5f5f4", "stone-200": "#e7e5e4", "stone-300": "#d6d3d1",
  "stone-400": "#a8a29e", "stone-500": "#78716c", "stone-600": "#57534e", "stone-700": "#44403c",
  "stone-800": "#292524", "stone-900": "#1c1917", "stone-950": "#0c0a09",
  // Red
  "red-50": "#fef2f2", "red-100": "#fee2e2", "red-200": "#fecaca", "red-300": "#fca5a5",
  "red-400": "#f87171", "red-500": "#ef4444", "red-600": "#dc2626", "red-700": "#b91c1c",
  "red-800": "#991b1b", "red-900": "#7f1d1d", "red-950": "#450a0a",
  // Orange
  "orange-50": "#fff7ed", "orange-100": "#ffedd5", "orange-200": "#fed7aa", "orange-300": "#fdba74",
  "orange-400": "#fb923c", "orange-500": "#f97316", "orange-600": "#ea580c", "orange-700": "#c2410c",
  "orange-800": "#9a3412", "orange-900": "#7c2d12", "orange-950": "#431407",
  // Amber
  "amber-50": "#fffbeb", "amber-100": "#fef3c7", "amber-200": "#fde68a", "amber-300": "#fcd34d",
  "amber-400": "#fbbf24", "amber-500": "#f59e0b", "amber-600": "#d97706", "amber-700": "#b45309",
  "amber-800": "#92400e", "amber-900": "#78350f", "amber-950": "#451a03",
  // Yellow
  "yellow-50": "#fefce8", "yellow-100": "#fef9c3", "yellow-200": "#fef08a", "yellow-300": "#fde047",
  "yellow-400": "#facc15", "yellow-500": "#eab308", "yellow-600": "#ca8a04", "yellow-700": "#a16207",
  "yellow-800": "#854d0e", "yellow-900": "#713f12", "yellow-950": "#422006",
  // Lime
  "lime-50": "#f7fee7", "lime-100": "#ecfccb", "lime-200": "#d9f99d", "lime-300": "#bef264",
  "lime-400": "#a3e635", "lime-500": "#84cc16", "lime-600": "#65a30d", "lime-700": "#4d7c0f",
  "lime-800": "#3f6212", "lime-900": "#365314", "lime-950": "#1a2e05",
  // Green
  "green-50": "#f0fdf4", "green-100": "#dcfce7", "green-200": "#bbf7d0", "green-300": "#86efac",
  "green-400": "#4ade80", "green-500": "#22c55e", "green-600": "#16a34a", "green-700": "#15803d",
  "green-800": "#166534", "green-900": "#14532d", "green-950": "#052e16",
  // Emerald
  "emerald-50": "#ecfdf5", "emerald-100": "#d1fae5", "emerald-200": "#a7f3d0", "emerald-300": "#6ee7b7",
  "emerald-400": "#34d399", "emerald-500": "#10b981", "emerald-600": "#059669", "emerald-700": "#047857",
  "emerald-800": "#065f46", "emerald-900": "#064e3b", "emerald-950": "#022c22",
  // Teal
  "teal-50": "#f0fdfa", "teal-100": "#ccfbf1", "teal-200": "#99f6e4", "teal-300": "#5eead4",
  "teal-400": "#2dd4bf", "teal-500": "#14b8a6", "teal-600": "#0d9488", "teal-700": "#0f766e",
  "teal-800": "#115e59", "teal-900": "#134e4a", "teal-950": "#042f2e",
  // Cyan
  "cyan-50": "#ecfeff", "cyan-100": "#cffafe", "cyan-200": "#a5f3fc", "cyan-300": "#67e8f9",
  "cyan-400": "#22d3ee", "cyan-500": "#06b6d4", "cyan-600": "#0891b2", "cyan-700": "#0e7490",
  "cyan-800": "#155e75", "cyan-900": "#164e63", "cyan-950": "#083344",
  // Sky
  "sky-50": "#f0f9ff", "sky-100": "#e0f2fe", "sky-200": "#bae6fd", "sky-300": "#7dd3fc",
  "sky-400": "#38bdf8", "sky-500": "#0ea5e9", "sky-600": "#0284c7", "sky-700": "#0369a1",
  "sky-800": "#075985", "sky-900": "#0c4a6e", "sky-950": "#082f49",
  // Blue
  "blue-50": "#eff6ff", "blue-100": "#dbeafe", "blue-200": "#bfdbfe", "blue-300": "#93c5fd",
  "blue-400": "#60a5fa", "blue-500": "#3b82f6", "blue-600": "#2563eb", "blue-700": "#1d4ed8",
  "blue-800": "#1e40af", "blue-900": "#1e3a8a", "blue-950": "#172554",
  // Indigo
  "indigo-50": "#eef2ff", "indigo-100": "#e0e7ff", "indigo-200": "#c7d2fe", "indigo-300": "#a5b4fc",
  "indigo-400": "#818cf8", "indigo-500": "#6366f1", "indigo-600": "#4f46e5", "indigo-700": "#4338ca",
  "indigo-800": "#3730a3", "indigo-900": "#312e81", "indigo-950": "#1e1b4b",
  // Violet
  "violet-50": "#f5f3ff", "violet-100": "#ede9fe", "violet-200": "#ddd6fe", "violet-300": "#c4b5fd",
  "violet-400": "#a78bfa", "violet-500": "#8b5cf6", "violet-600": "#7c3aed", "violet-700": "#6d28d9",
  "violet-800": "#5b21b6", "violet-900": "#4c1d95", "violet-950": "#2e1065",
  // Purple
  "purple-50": "#faf5ff", "purple-100": "#f3e8ff", "purple-200": "#e9d5ff", "purple-300": "#d8b4fe",
  "purple-400": "#c084fc", "purple-500": "#a855f7", "purple-600": "#9333ea", "purple-700": "#7e22ce",
  "purple-800": "#6b21a8", "purple-900": "#581c87", "purple-950": "#3b0764",
  // Fuchsia
  "fuchsia-50": "#fdf4ff", "fuchsia-100": "#fae8ff", "fuchsia-200": "#f5d0fe", "fuchsia-300": "#f0abfc",
  "fuchsia-400": "#e879f9", "fuchsia-500": "#d946ef", "fuchsia-600": "#c026d3", "fuchsia-700": "#a21caf",
  "fuchsia-800": "#86198f", "fuchsia-900": "#701a75", "fuchsia-950": "#4a044e",
  // Pink
  "pink-50": "#fdf2f8", "pink-100": "#fce7f3", "pink-200": "#fbcfe8", "pink-300": "#f9a8d4",
  "pink-400": "#f472b6", "pink-500": "#ec4899", "pink-600": "#db2777", "pink-700": "#be185d",
  "pink-800": "#9d174d", "pink-900": "#831843", "pink-950": "#500724",
  // Rose
  "rose-50": "#fff1f2", "rose-100": "#ffe4e6", "rose-200": "#fecdd3", "rose-300": "#fda4af",
  "rose-400": "#fb7185", "rose-500": "#f43f5e", "rose-600": "#e11d48", "rose-700": "#be123c",
  "rose-800": "#9f1239", "rose-900": "#881337", "rose-950": "#4c0519",
};

// Font weight values for segmented control (all Tailwind weights)
const FONT_WEIGHT_VALUES = ["thin", "extralight", "light", "normal", "medium", "semibold", "bold", "extrabold", "black"];

// Display options
const DISPLAY_OPTIONS = [
  { value: "block", icon: "block", label: "Block" },
  { value: "flex", icon: "flex", label: "Flex" },
  { value: "grid", icon: "grid", label: "Grid" },
  { value: "inline", icon: "inline", label: "Inline" },
  { value: "inline-block", icon: "inline-block", label: "Inline Block" },
  { value: "hidden", icon: "hidden", label: "Hidden" },
];

// Flex direction options
const FLEX_DIRECTION_OPTIONS = [
  { value: "row", icon: "→", label: "Row" },
  { value: "row-reverse", icon: "←", label: "Row Reverse" },
  { value: "col", icon: "↓", label: "Column" },
  { value: "col-reverse", icon: "↑", label: "Col Reverse" },
];

// Justify content options
const JUSTIFY_OPTIONS = [
  { value: "start", icon: "start", label: "Start" },
  { value: "center", icon: "center", label: "Center" },
  { value: "end", icon: "end", label: "End" },
  { value: "between", icon: "between", label: "Between" },
  { value: "around", icon: "around", label: "Around" },
  { value: "evenly", icon: "evenly", label: "Evenly" },
];

// Align items options
const ALIGN_OPTIONS = [
  { value: "start", icon: "start", label: "Start" },
  { value: "center", icon: "center", label: "Center" },
  { value: "end", icon: "end", label: "End" },
  { value: "stretch", icon: "stretch", label: "Stretch" },
  { value: "baseline", icon: "baseline", label: "Baseline" },
];

// =============================================================================
// Reusable UI Components
// =============================================================================

/**
 * Render a segmented control (button group)
 * @param {string} id - Unique identifier
 * @param {Array} options - Array of { value, icon?, label }
 * @param {string} currentValue - Currently selected value
 * @param {string} prop - Property name for data attribute
 */
function renderSegmentedControl(id, options, currentValue, prop) {
  return `
    <div class="segmented-control" data-control="${id}" data-prop="${prop}">
      ${options.map(opt => `
        <button class="seg-btn ${currentValue === opt.value ? 'active' : ''}" 
                data-value="${opt.value}" 
                title="${opt.label}">
          ${opt.icon ? `<span class="seg-icon">${opt.icon}</span>` : ''}
          ${!opt.icon ? `<span class="seg-label">${opt.label}</span>` : ''}
        </button>
      `).join('')}
    </div>
  `;
}

/**
 * Render a select dropdown with optional custom value support
 * @param {string} id - Unique identifier
 * @param {Array} options - Array of { value, label } or just values
 * @param {string} currentValue - Currently selected value
 * @param {string} prop - Property name
 * @param {boolean} allowCustom - Allow arbitrary values
 */
function renderSelect(id, options, currentValue, prop, allowCustom = false) {
  const isCustom = currentValue && !options.find(o => 
    (typeof o === 'string' ? o : o.value) === currentValue
  );
  
  return `
    <div class="select-wrapper" data-control="${id}" data-prop="${prop}">
      <select class="ctrl-select" ${allowCustom ? 'data-allow-custom="true"' : ''}>
        <option value="">–</option>
        ${options.map(opt => {
          const value = typeof opt === 'string' ? opt : opt.value;
          const label = typeof opt === 'string' ? opt : (opt.label || opt.value);
          return `<option value="${value}" ${currentValue === value ? 'selected' : ''}>${label}</option>`;
        }).join('')}
        ${isCustom ? `<option value="${currentValue}" selected>${currentValue}</option>` : ''}
      </select>
      ${allowCustom ? `
        <input type="text" class="select-custom" placeholder="Custom..." 
               value="${isCustom ? currentValue : ''}" 
               style="display: ${isCustom ? 'block' : 'none'}">
      ` : ''}
    </div>
  `;
}

/**
 * Render a color picker with preset swatches
 * @param {string} id - Unique identifier  
 * @param {string} currentValue - Current color value (Tailwind class or hex)
 * @param {string} prop - Property name
 * @param {Array} presets - Color presets to show
 */
function renderColorPicker(id, currentValue, prop, presets = null) {
  const defaultPresets = [
    "transparent", "white", "black",
    "slate-100", "slate-500", "slate-900",
    "red-500", "orange-500", "yellow-500", 
    "green-500", "blue-500", "purple-500"
  ];
  const colors = presets || defaultPresets;
  
  // Try to convert Tailwind color to hex for the picker
  const hexValue = TAILWIND_COLORS[currentValue] || currentValue || '#000000';
  const isValidHex = /^#[0-9a-fA-F]{6}$/.test(hexValue);
  
  return `
    <div class="color-picker" data-control="${id}" data-prop="${prop}">
      <div class="color-preview-row">
        <input type="color" class="color-input" value="${isValidHex ? hexValue : '#000000'}">
        <input type="text" class="color-text" value="${currentValue || ''}" placeholder="transparent">
      </div>
      <div class="color-swatches">
        ${colors.map(color => {
          const hex = TAILWIND_COLORS[color] || color;
          const isTransparent = color === 'transparent';
          return `
            <button class="color-swatch ${currentValue === color ? 'active' : ''} ${isTransparent ? 'transparent' : ''}" 
                    data-color="${color}" 
                    title="${color}"
                    style="background: ${isTransparent ? 'transparent' : hex}">
              ${isTransparent ? '∅' : ''}
            </button>
          `;
        }).join('')}
      </div>
    </div>
  `;
}

/**
 * Render a numeric input with optional stepper
 * @param {string} id - Unique identifier
 * @param {string} currentValue - Current value
 * @param {string} prop - Property name
 * @param {string} placeholder - Placeholder text
 * @param {Array} presets - Quick preset values
 */
function renderNumericInput(id, currentValue, prop, placeholder = "auto", presets = null) {
  return `
    <div class="numeric-input" data-control="${id}" data-prop="${prop}">
      <input type="text" class="num-input" value="${currentValue || ''}" placeholder="${placeholder}">
      ${presets ? `
        <div class="num-presets">
          ${presets.map(p => `
            <button class="num-preset ${currentValue === p ? 'active' : ''}" data-value="${p}">${p}</button>
          `).join('')}
        </div>
      ` : ''}
    </div>
  `;
}

/**
 * Render a slider control
 * @param {string} id - Unique identifier
 * @param {string} currentValue - Current value
 * @param {string} prop - Property name
 * @param {Array} values - Array of possible values
 */
function renderSlider(id, currentValue, prop, values) {
  const currentIdx = values.indexOf(currentValue);
  const percent = currentIdx >= 0 ? (currentIdx / (values.length - 1)) * 100 : 50;
  
  return `
    <div class="slider-control" data-control="${id}" data-prop="${prop}">
      <input type="range" class="slider-input" min="0" max="${values.length - 1}" 
             value="${currentIdx >= 0 ? currentIdx : Math.floor(values.length / 2)}"
             style="--percent: ${percent}%">
      <span class="slider-value">${currentValue || values[Math.floor(values.length / 2)]}</span>
    </div>
  `;
}

// =============================================================================
// Tailwind Class Parsing & Manipulation
// =============================================================================

/**
 * Parse Tailwind classes to extract spacing values
 * Returns an object with current values for the active breakpoint/state
 */
function parseSpacingClasses(classString) {
  if (!classString) return {};
  
  const classes = classString.split(/\s+/).filter(Boolean);
  const targetPrefix = getActivePrefixForClass(); // e.g., "md:hover:" or "md:" or ""
  
  const result = {
    p: null, px: null, py: null, pt: null, pr: null, pb: null, pl: null,
    m: null, mx: null, my: null, mt: null, mr: null, mb: null, ml: null,
  };
  
  // Regex to match spacing classes with optional prefix(es)
  // e.g., "p-4", "md:px-2", "hover:m-auto", "md:hover:p-[20px]", "-m-4"
  // Groups: 1=prefix (everything before the prop), 2=prop (p/m with optional x/y/t/r/b/l), 3=value
  const spacingRegex = /^((?:[a-z0-9]+:)*)(-?(?:p|m)(?:x|y|t|r|b|l)?)-(.+)$/;
  
  for (const cls of classes) {
    const match = cls.match(spacingRegex);
    if (!match) continue;
    
    const [, classPrefix, prop, value] = match;
    
    // Check if this class matches our current context
    if (classPrefix === targetPrefix) {
      // Extract the property name (p, px, py, pt, pr, pb, pl, m, mx, etc.)
      // Remove leading dash for negative margins
      const propName = prop.replace(/^-/, "");
      result[propName] = prop.startsWith("-") ? `-${value}` : value;
    }
  }
  
  return result;
}

/**
 * Get the prefix string for the current breakpoint/state context
 * e.g., "md:hover:" or "lg:" or "hover:" or ""
 */
function getActivePrefixForClass() {
  const bp = State.activeBreakpoint;
  const st = State.activeState;
  
  let prefix = "";
  if (bp !== "base") prefix += bp + ":";
  if (st !== "normal") prefix += st + ":";
  
  return prefix;
}

/**
 * Update a spacing class in the class string
 * @param {string} classString - Current classes
 * @param {string} prop - Property to update (p, px, py, pt, pr, pb, pl, m, mx, my, mt, mr, mb, ml)
 * @param {string|null} value - New value (null to remove)
 * @returns {string} Updated class string
 */
function updateSpacingClass(classString, prop, value) {
  const classes = (classString || "").split(/\s+/).filter(Boolean);
  const prefix = getActivePrefixForClass();
  const fullPrefix = prefix ? prefix : "";
  
  // Build regex to find existing class with same prefix and property
  const prefixPattern = prefix ? prefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  const propPattern = prop.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const regex = new RegExp(`^${prefixPattern}${propPattern}-`);
  
  // Remove any existing class with this prefix+property
  const filtered = classes.filter(cls => !regex.test(cls));
  
  // Add new class if value is provided
  if (value !== null && value !== "") {
    filtered.push(`${fullPrefix}${prop}-${value}`);
  }
  
  return filtered.join(" ");
}

/**
 * Convert a pixel value to the nearest Tailwind spacing value
 */
function pxToTailwind(px) {
  // Check for exact match first
  if (PX_TO_SPACING[px] !== undefined) {
    return PX_TO_SPACING[px];
  }
  
  // Find nearest value
  const pxValues = Object.keys(PX_TO_SPACING).map(Number).sort((a, b) => a - b);
  let nearest = pxValues[0];
  let nearestDiff = Math.abs(px - nearest);
  
  for (const pxVal of pxValues) {
    const diff = Math.abs(px - pxVal);
    if (diff < nearestDiff) {
      nearest = pxVal;
      nearestDiff = diff;
    }
  }
  
  return PX_TO_SPACING[nearest];
}

/**
 * Convert a Tailwind spacing value to pixels
 */
function tailwindToPx(value) {
  if (value === "auto") return "auto";
  
  // Handle arbitrary values like [20px]
  if (value.startsWith("[") && value.endsWith("]")) {
    const inner = value.slice(1, -1);
    if (inner.endsWith("px")) {
      return parseInt(inner);
    }
    return inner;
  }
  
  return SPACING_SCALE[value] ?? value;
}

/**
 * Step to the next/previous value in the spacing scale
 */
function stepSpacingValue(currentValue, direction) {
  if (currentValue === null || currentValue === "") {
    return direction > 0 ? "0" : "0";
  }
  
  // Handle auto
  if (currentValue === "auto") {
    return direction > 0 ? "0" : "96";
  }
  
  // Handle arbitrary values
  if (currentValue.startsWith("[")) {
    const px = parseInt(currentValue.slice(1, -1));
    const nearestTw = pxToTailwind(px);
    const idx = SPACING_VALUES.indexOf(nearestTw);
    if (idx === -1) return currentValue;
    const newIdx = Math.max(0, Math.min(SPACING_VALUES.length - 1, idx + direction));
    return SPACING_VALUES[newIdx];
  }
  
  const idx = SPACING_VALUES.indexOf(currentValue);
  if (idx === -1) return currentValue;
  
  const newIdx = Math.max(0, Math.min(SPACING_VALUES.length - 1, idx + direction));
  return SPACING_VALUES[newIdx];
}

/**
 * Check if any classes exist for a property at other breakpoints/states
 * Returns array of prefixes that have values
 */
function getOtherPrefixesWithValue(classString, prop) {
  if (!classString) return [];
  
  const classes = classString.split(/\s+/).filter(Boolean);
  const currentPrefix = getActivePrefixForClass();
  const prefixes = [];
  
  const propPattern = prop.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const regex = new RegExp(`^((?:[a-z0-9]+:)*)${propPattern}-`);
  
  for (const cls of classes) {
    const match = cls.match(regex);
    if (match) {
      const prefix = match[1] || "";
      if (prefix !== currentPrefix && !prefixes.includes(prefix)) {
        prefixes.push(prefix);
      }
    }
  }
  
  return prefixes;
}

/**
 * Get the inherited value for a property (from base or lower breakpoints)
 */
function getInheritedValue(classString, prop) {
  if (!classString) return null;
  
  const classes = classString.split(/\s+/).filter(Boolean);
  const bp = State.activeBreakpoint;
  const st = State.activeState;
  
  // Build inheritance order - we check from most specific that's less than current, down to base
  // For md:hover, check: md: (without hover), hover: (at base), then base
  // For md:, check: sm:, then base
  // For hover:, check: base
  const inheritanceOrder = [];
  
  if (st !== "normal" && bp !== "base") {
    // e.g., for md:hover, check: md (without hover), hover (at base), then base
    inheritanceOrder.push({ bp, st: "normal" }); // md: alone
    inheritanceOrder.push({ bp: "base", st }); // hover: alone
    inheritanceOrder.push({ bp: "base", st: "normal" }); // base
  } else if (st !== "normal") {
    // e.g., for hover, check: base
    inheritanceOrder.push({ bp: "base", st: "normal" }); // base
  } else if (bp !== "base") {
    // e.g., for md, check breakpoints in order then base
    const bpOrder = ["sm", "md", "lg", "xl", "2xl"];
    const currentIdx = bpOrder.indexOf(bp);
    for (let i = currentIdx - 1; i >= 0; i--) {
      inheritanceOrder.push({ bp: bpOrder[i], st: "normal" });
    }
    inheritanceOrder.push({ bp: "base", st: "normal" }); // base
  }
  
  const propPattern = prop.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  
  for (const ctx of inheritanceOrder) {
    // Build the prefix for this context
    let prefix = "";
    if (ctx.bp !== "base") prefix += ctx.bp + ":";
    if (ctx.st !== "normal") prefix += ctx.st + ":";
    
    const escapedPrefix = prefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const regex = new RegExp(`^${escapedPrefix}${propPattern}-(.+)$`);
    
    for (const cls of classes) {
      const match = cls.match(regex);
      if (match) {
        const fromLabel = prefix ? prefix.slice(0, -1) : "base";
        return { value: match[1], from: fromLabel };
      }
    }
  }
  
  return null;
}

/**
 * Parse a single property class from the class string
 * @param {string} classString - The full class string
 * @param {string} propPrefix - The property prefix (e.g., "w", "h", "text", "bg")
 * @param {boolean} exactMatch - If true, match exact prefix, otherwise allow variations
 */
function parsePropertyClass(classString, propPrefix, exactMatch = false) {
  if (!classString) return null;
  
  const classes = classString.split(/\s+/).filter(Boolean);
  const targetPrefix = getActivePrefixForClass();
  
  // Build regex pattern
  const prefixPattern = targetPrefix ? targetPrefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  const propPattern = propPrefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const regex = exactMatch 
    ? new RegExp(`^${prefixPattern}${propPattern}-(.+)$`)
    : new RegExp(`^${prefixPattern}${propPattern}(?:-(.+))?$`);
  
  for (const cls of classes) {
    const match = cls.match(regex);
    if (match) {
      return match[1] || "";
    }
  }
  
  return null;
}

/**
 * Parse display class
 */
function parseDisplayClass(classString) {
  if (!classString) return null;
  
  const classes = classString.split(/\s+/).filter(Boolean);
  const targetPrefix = getActivePrefixForClass();
  const prefixPattern = targetPrefix ? targetPrefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  
  // Display classes don't have a prefix like "display-block", they're just "block", "flex", etc.
  const displayClasses = ["block", "inline-block", "inline", "flex", "inline-flex", "grid", "inline-grid", "hidden", "contents", "flow-root"];
  
  for (const cls of classes) {
    for (const dc of displayClasses) {
      const regex = new RegExp(`^${prefixPattern}${dc.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}$`);
      if (regex.test(cls)) {
        return dc;
      }
    }
  }
  
  return null;
}

/**
 * Update a display class
 */
function updateDisplayClass(classString, value) {
  const classes = (classString || "").split(/\s+/).filter(Boolean);
  const prefix = getActivePrefixForClass();
  
  // Remove existing display classes with this prefix
  const displayClasses = ["block", "inline-block", "inline", "flex", "inline-flex", "grid", "inline-grid", "hidden", "contents", "flow-root"];
  const prefixPattern = prefix ? prefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  
  const filtered = classes.filter(cls => {
    for (const dc of displayClasses) {
      const regex = new RegExp(`^${prefixPattern}${dc.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}$`);
      if (regex.test(cls)) return false;
    }
    return true;
  });
  
  // Add new class if value provided
  if (value) {
    filtered.push(`${prefix}${value}`);
  }
  
  return filtered.join(" ");
}

/**
 * Generic function to update a property class
 * @param {string} classString - Current classes
 * @param {string} propPrefix - Property prefix (e.g., "w", "h", "rounded")
 * @param {string|null} value - New value (null to remove)
 */
function updatePropertyClass(classString, propPrefix, value) {
  const classes = (classString || "").split(/\s+/).filter(Boolean);
  const prefix = getActivePrefixForClass();
  
  // Build regex to find existing class
  const prefixPattern = prefix ? prefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  const propPattern = propPrefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const regex = new RegExp(`^${prefixPattern}${propPattern}(?:-|$)`);
  
  // Remove existing
  const filtered = classes.filter(cls => !regex.test(cls));
  
  // Add new
  if (value !== null && value !== "") {
    // Some properties like "rounded" can have empty value for default
    if (value === "" && ["rounded", "shadow", "border"].includes(propPrefix)) {
      filtered.push(`${prefix}${propPrefix}`);
    } else {
      filtered.push(`${prefix}${propPrefix}-${value}`);
    }
  }
  
  return filtered.join(" ");
}

/**
 * Parse layout-related classes (flex-direction, justify, align, gap)
 */
function parseLayoutClasses(classString) {
  return {
    display: parseDisplayClass(classString),
    flexDirection: parsePropertyClass(classString, "flex-row") ? "row" :
                   parsePropertyClass(classString, "flex-row-reverse") !== null ? "row-reverse" :
                   parsePropertyClass(classString, "flex-col-reverse") !== null ? "col-reverse" :
                   parsePropertyClass(classString, "flex-col") !== null ? "col" : null,
    justifyContent: parsePropertyClass(classString, "justify"),
    alignItems: parsePropertyClass(classString, "items"),
    gap: parsePropertyClass(classString, "gap"),
    gapX: parsePropertyClass(classString, "gap-x"),
    gapY: parsePropertyClass(classString, "gap-y"),
  };
}

/**
 * Update flex direction class
 */
function updateFlexDirectionClass(classString, value) {
  const classes = (classString || "").split(/\s+/).filter(Boolean);
  const prefix = getActivePrefixForClass();
  const prefixPattern = prefix ? prefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  
  // Remove all flex-direction classes
  const dirClasses = ["flex-row", "flex-row-reverse", "flex-col", "flex-col-reverse"];
  const filtered = classes.filter(cls => {
    for (const dc of dirClasses) {
      const regex = new RegExp(`^${prefixPattern}${dc}$`);
      if (regex.test(cls)) return false;
    }
    return true;
  });
  
  // Add new
  if (value) {
    filtered.push(`${prefix}flex-${value}`);
  }
  
  return filtered.join(" ");
}

/**
 * Parse size classes (w, h, min-w, max-w, min-h, max-h)
 */
function parseSizeClasses(classString) {
  return {
    w: parsePropertyClass(classString, "w"),
    h: parsePropertyClass(classString, "h"),
    minW: parsePropertyClass(classString, "min-w"),
    maxW: parsePropertyClass(classString, "max-w"),
    minH: parsePropertyClass(classString, "min-h"),
    maxH: parsePropertyClass(classString, "max-h"),
  };
}

/**
 * Parse typography classes
 */
function parseTypographyClasses(classString) {
  return {
    fontSize: parsePropertyClass(classString, "text", true),
    fontWeight: parsePropertyClass(classString, "font"),
    textColor: parseTextColorClass(classString),
    textAlign: parsePropertyClass(classString, "text-left") !== null ? "left" :
               parsePropertyClass(classString, "text-center") !== null ? "center" :
               parsePropertyClass(classString, "text-right") !== null ? "right" :
               parsePropertyClass(classString, "text-justify") !== null ? "justify" : null,
  };
}

/**
 * Parse text color (special handling because text-* can be size or color)
 */
function parseTextColorClass(classString) {
  if (!classString) return null;
  
  const classes = classString.split(/\s+/).filter(Boolean);
  const targetPrefix = getActivePrefixForClass();
  const prefixPattern = targetPrefix ? targetPrefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  
  // Text sizes to exclude
  const textSizes = ["xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl", "9xl"];
  const textAligns = ["left", "center", "right", "justify"];
  
  for (const cls of classes) {
    const regex = new RegExp(`^${prefixPattern}text-(.+)$`);
    const match = cls.match(regex);
    if (match) {
      const value = match[1];
      // Skip if it's a size or alignment
      if (!textSizes.includes(value) && !textAligns.includes(value)) {
        return value;
      }
    }
  }
  
  return null;
}

/**
 * Update text color class
 */
function updateTextColorClass(classString, value) {
  const classes = (classString || "").split(/\s+/).filter(Boolean);
  const prefix = getActivePrefixForClass();
  const prefixPattern = prefix ? prefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  
  // Text sizes and aligns to preserve
  const textSizes = ["xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl", "9xl"];
  const textAligns = ["left", "center", "right", "justify"];
  
  // Remove existing text color (but keep sizes and aligns)
  const filtered = classes.filter(cls => {
    const regex = new RegExp(`^${prefixPattern}text-(.+)$`);
    const match = cls.match(regex);
    if (match) {
      const val = match[1];
      return textSizes.includes(val) || textAligns.includes(val);
    }
    return true;
  });
  
  // Add new color
  if (value) {
    filtered.push(`${prefix}text-${value}`);
  }
  
  return filtered.join(" ");
}

/**
 * Update text alignment class
 */
function updateTextAlignClass(classString, value) {
  const classes = (classString || "").split(/\s+/).filter(Boolean);
  const prefix = getActivePrefixForClass();
  const prefixPattern = prefix ? prefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  
  // Remove existing alignment
  const alignClasses = ["text-left", "text-center", "text-right", "text-justify"];
  const filtered = classes.filter(cls => {
    for (const ac of alignClasses) {
      const regex = new RegExp(`^${prefixPattern}${ac}$`);
      if (regex.test(cls)) return false;
    }
    return true;
  });
  
  // Add new
  if (value) {
    filtered.push(`${prefix}text-${value}`);
  }
  
  return filtered.join(" ");
}

/**
 * Parse background classes
 */
function parseBgColorClass(classString) {
  return parsePropertyClass(classString, "bg");
}

/**
 * Parse border classes
 */
function parseBorderClasses(classString) {
  return {
    rounded: parsePropertyClass(classString, "rounded"),
    borderWidth: parsePropertyClass(classString, "border"),
    borderColor: parsePropertyClass(classString, "border", true),
  };
}

/**
 * Parse effects classes
 */
function parseEffectsClasses(classString) {
  return {
    opacity: parsePropertyClass(classString, "opacity"),
    shadow: parsePropertyClass(classString, "shadow"),
  };
}

/**
 * Parse position classes
 */
function parsePositionClasses(classString) {
  if (!classString) return { position: null, inset: null, top: null, right: null, bottom: null, left: null, zIndex: null };
  
  const classes = classString.split(/\s+/).filter(Boolean);
  const targetPrefix = getActivePrefixForClass();
  const prefixPattern = targetPrefix ? targetPrefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  
  let position = null;
  const positionTypes = ["static", "relative", "absolute", "fixed", "sticky"];
  
  for (const cls of classes) {
    for (const pos of positionTypes) {
      const regex = new RegExp(`^${prefixPattern}${pos}$`);
      if (regex.test(cls)) {
        position = pos;
        break;
      }
    }
  }
  
  return {
    position,
    inset: parsePropertyClass(classString, "inset"),
    top: parsePropertyClass(classString, "top"),
    right: parsePropertyClass(classString, "right"),
    bottom: parsePropertyClass(classString, "bottom"),
    left: parsePropertyClass(classString, "left"),
    zIndex: parsePropertyClass(classString, "z"),
  };
}

/**
 * Update position type class
 */
function updatePositionClass(classString, value) {
  const classes = (classString || "").split(/\s+/).filter(Boolean);
  const prefix = getActivePrefixForClass();
  const prefixPattern = prefix ? prefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  
  // Remove existing position class
  const positionTypes = ["static", "relative", "absolute", "fixed", "sticky"];
  const filtered = classes.filter(cls => {
    for (const pos of positionTypes) {
      const regex = new RegExp(`^${prefixPattern}${pos}$`);
      if (regex.test(cls)) return false;
    }
    return true;
  });
  
  // Add new position class
  if (value && value !== "static") {
    filtered.push(`${prefix}${value}`);
  }
  
  return filtered.join(" ");
}

/**
 * Parse overflow classes
 */
function parseOverflowClasses(classString) {
  if (!classString) return { overflow: null, overflowX: null, overflowY: null };
  
  return {
    overflow: parsePropertyClass(classString, "overflow"),
    overflowX: parsePropertyClass(classString, "overflow-x"),
    overflowY: parsePropertyClass(classString, "overflow-y"),
  };
}

/**
 * Update overflow class
 */
function updateOverflowClass(classString, axis, value) {
  const classes = (classString || "").split(/\s+/).filter(Boolean);
  const prefix = getActivePrefixForClass();
  const prefixPattern = prefix ? prefix.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') : "";
  
  // Build the property name based on axis
  const propName = axis ? `overflow-${axis}` : "overflow";
  
  // Remove existing overflow class for this axis
  const filtered = classes.filter(cls => {
    const regex = new RegExp(`^${prefixPattern}${propName}-(auto|hidden|clip|visible|scroll)$`);
    return !regex.test(cls);
  });
  
  // Add new value
  if (value) {
    filtered.push(`${prefix}${propName}-${value}`);
  }
  
  return filtered.join(" ");
}

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
    
    <div id="context-bar">
      <div class="context-group">
        <span class="context-label">Screen</span>
        <div class="context-buttons" id="breakpoint-buttons">
          <button class="ctx-btn active" data-bp="base" title="All sizes">*</button>
          <button class="ctx-btn" data-bp="sm" title="640px+">sm</button>
          <button class="ctx-btn" data-bp="md" title="768px+">md</button>
          <button class="ctx-btn" data-bp="lg" title="1024px+">lg</button>
          <button class="ctx-btn" data-bp="xl" title="1280px+">xl</button>
        </div>
      </div>
      <div class="context-divider"></div>
      <div class="context-group">
        <span class="context-label">State</span>
        <div class="context-buttons" id="state-buttons">
          <button class="ctx-btn active" data-state="normal" title="Normal state">–</button>
          <button class="ctx-btn" data-state="hover" title="Hover state">:hover</button>
          <button class="ctx-btn" data-state="focus" title="Focus state">:focus</button>
          <button class="ctx-btn" data-state="active" title="Active state">:active</button>
        </div>
      </div>
    </div>
    
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
    
    <div id="apply-footer" class="apply-footer hidden">
      <button id="apply-btn">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="20,6 9,17 4,12"/>
        </svg>
        Apply Changes
        <kbd>⌘↵</kbd>
      </button>
    </div>
    
    <footer id="footer">
      <div class="shortcut-hint">
        <kbd>⌘</kbd><kbd>.</kbd> toggle · <kbd>S</kbd> select · <kbd>esc</kbd> close
      </div>
      <div class="version-hint">Rejoice v${window.__REJOICE_VERSION__ || "?"}</div>
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

  // Breakpoint switcher
  $$panel("#breakpoint-buttons .ctx-btn").forEach((btn) =>
    btn.addEventListener("click", () => switchBreakpoint(btn.dataset.bp)),
  );

  // State switcher
  $$panel("#state-buttons .ctx-btn").forEach((btn) =>
    btn.addEventListener("click", () => switchState(btn.dataset.state)),
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
  // Don't allow select mode when isolated
  if (State.isolatedComponent) return;
  
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

function switchBreakpoint(bp) {
  State.activeBreakpoint = bp;
  
  // Update UI
  $$panel("#breakpoint-buttons .ctx-btn").forEach((btn) =>
    btn.classList.toggle("active", btn.dataset.bp === bp),
  );
  
  // Resize canvas to match breakpoint
  const canvas = $("#canvas");
  const maxWidth = BREAKPOINTS[bp];
  
  if (maxWidth) {
    canvas.style.maxWidth = maxWidth + "px";
    canvas.style.margin = "0 auto";
    $("#studio").classList.add("breakpoint-active");
  } else {
    canvas.style.maxWidth = "";
    canvas.style.margin = "";
    $("#studio").classList.remove("breakpoint-active");
  }
  
  // Re-render inspect panel to show correct values for this breakpoint
  renderInspect();
}

function switchState(state) {
  State.activeState = state;
  
  // Update UI
  $$panel("#state-buttons .ctx-btn").forEach((btn) =>
    btn.classList.toggle("active", btn.dataset.state === state),
  );
  
  // Tell the bridge to force this state on the selected element
  if (State.selectedElement) {
    send({ 
      type: "force-state", 
      path: State.selectedElement.path, 
      state: state === "normal" ? null : state 
    });
  }
  
  // Re-render inspect panel to show correct values for this state
  renderInspect();
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

  // Build the design tools sections
  const layoutSection = renderLayoutSection(el.classes);
  const sizeSection = renderSizeSection(el.classes);
  const positionSection = renderPositionSection(el.classes);
  const overflowSection = renderOverflowSection(el.classes);
  const spacingSection = renderSpacingSection(el.classes);
  const typographySection = renderTypographySection(el.classes);
  const backgroundSection = renderBackgroundSection(el.classes);
  const bordersSection = renderBordersSection(el.classes);
  const effectsSection = renderEffectsSection(el.classes);
  const classesSection = renderClassesSection(el.classes, isComponentRoot, isInComponent, el.tagName);

  content.innerHTML = `
    ${elementHeader}
    
    ${isolateSection}
    
    <div class="design-sections">
      ${layoutSection}
      ${sizeSection}
      ${positionSection}
      ${overflowSection}
      ${spacingSection}
      ${typographySection}
      ${backgroundSection}
      ${bordersSection}
      ${effectsSection}
      ${classesSection}
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

  // Bind collapsible sections
  bindCollapsibleSections();
  
  // Bind all design controls
  bindDesignControls();

  // Bind classes input and apply button
  bindClassesAndApply();
}

function renderClassesSection(classes, isComponentRoot, isInComponent, tagName) {
  const isCollapsed = getSectionCollapsed("classes");
  const label = isComponentRoot
    ? `Classes <span class="section-target">on component</span>`
    : isInComponent
      ? `Classes <span class="section-target">on &lt;${tagName}&gt;</span>`
      : "Classes";

  return `
    <div class="design-section ${isCollapsed ? "" : "expanded"}" data-section="classes">
      <div class="design-section-header" data-section="classes">
        <svg class="section-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M16 4h2a2 2 0 012 2v14a2 2 0 01-2 2H6a2 2 0 01-2-2V6a2 2 0 012-2h2"/>
          <rect x="8" y="2" width="8" height="4" rx="1"/>
        </svg>
        <span class="section-label">${label}</span>
        <svg class="section-chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="6,9 12,15 18,9"/>
        </svg>
      </div>
      <div class="design-section-body">
        <textarea id="classes-input" spellcheck="false" placeholder="p-4 flex items-center ...">${classes || ""}</textarea>
      </div>
    </div>
  `;
}

function bindClassesAndApply() {
  const el = State.selectedElement;
  if (!el) return;

  const applyBtn = $panel("#apply-btn");
  const applyFooter = $panel("#apply-footer");
  const classesInput = $panel("#classes-input");
  if (!applyBtn || !classesInput) return;

  // Track what's saved in the filesystem
  if (el.savedClasses === undefined) {
    el.savedClasses = el.classes || "";
  }

  function updateApplyState() {
    const hasChanges = el.classes !== el.savedClasses;
    applyFooter?.classList.toggle("hidden", !hasChanges);
  }

  // Update when input changes
  classesInput.addEventListener("input", () => {
    el.classes = classesInput.value;
    updateApplyState();
    
    // Instant preview
    send({ type: "preview-classes", path: el.path, classes: el.classes });
    
    // Update ALL design controls to stay in sync
    refreshAllDesignControls();
  });

  // Apply button
  applyBtn.addEventListener("click", () => {
    syncClassesToFile(el.classes);
  });

  // Keyboard shortcut
  classesInput.addEventListener("keydown", (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
      e.preventDefault();
      syncClassesToFile(el.classes);
    }
  });
  
  // Also listen for global cmd+enter when focus is anywhere in panel
  State.shadowRoot?.addEventListener("keydown", (e) => {
    if ((e.metaKey || e.ctrlKey) && e.key === "Enter" && el.classes !== el.savedClasses) {
      e.preventDefault();
      syncClassesToFile(el.classes);
    }
  });

  updateApplyState();
}

function refreshSpacingControls() {
  const el = State.selectedElement;
  if (!el) return;
  
  const spacingBody = $panel('[data-section="spacing"] .design-section-body');
  if (spacingBody) {
    const spacing = parseSpacingClasses(el.classes);
    spacingBody.innerHTML = `
      <div class="spacing-row">
        ${renderSpacingGroup(el.classes, spacing, "m", "M")}
        ${renderSpacingGroup(el.classes, spacing, "p", "P")}
      </div>
    `;
    bindSpacingControls();
  }
}

/**
 * Refresh ALL design controls to match current classes (for 2-way sync)
 */
function refreshAllDesignControls() {
  const el = State.selectedElement;
  if (!el) return;
  
  // Refresh spacing
  refreshSpacingControls();
  
  // Refresh layout controls
  const layout = parseLayoutClasses(el.classes);
  refreshSegmentedControl("display", layout.display);
  refreshSegmentedControl("flex-direction", layout.flexDirection);
  refreshSegmentedControl("justify", layout.justifyContent);
  refreshSegmentedControl("align", layout.alignItems);
  refreshNumericInput("gap", layout.gap);
  
  // Show/hide flex controls based on display value
  const flexControls = $$panel(".flex-controls");
  flexControls.forEach(fc => {
    fc.classList.toggle("hidden", layout.display !== "flex" && layout.display !== "grid");
  });
  
  // Refresh size controls
  const size = parseSizeClasses(el.classes);
  refreshNumericInput("width", size.w);
  refreshNumericInput("height", size.h);
  refreshNumericInput("min-w", size.minW);
  refreshNumericInput("max-w", size.maxW);
  refreshNumericInput("min-h", size.minH);
  refreshNumericInput("max-h", size.maxH);
  
  // Refresh position controls
  const pos = parsePositionClasses(el.classes);
  refreshSegmentedControl("position", pos.position || "static");
  refreshNumericInput("inset", pos.inset);
  refreshNumericInput("top", pos.top);
  refreshNumericInput("right", pos.right);
  refreshNumericInput("bottom", pos.bottom);
  refreshNumericInput("left", pos.left);
  refreshNumericInput("z-index", pos.zIndex);
  
  // Show/hide inset controls based on position
  const positionInsets = $$panel(".position-insets");
  positionInsets.forEach(pi => {
    pi.classList.toggle("hidden", !pos.position || pos.position === "static");
  });
  
  // Refresh overflow controls
  const overflow = parseOverflowClasses(el.classes);
  refreshSegmentedControl("overflow", overflow.overflow);
  refreshSegmentedControl("overflow-x", overflow.overflowX);
  refreshSegmentedControl("overflow-y", overflow.overflowY);
  
  // Refresh typography controls
  const typo = parseTypographyClasses(el.classes);
  refreshSelect("font-size", typo.fontSize);
  refreshSelect("font-weight", typo.fontWeight);
  refreshSegmentedControl("text-align", typo.textAlign);
  refreshColorPicker("text-color", typo.textColor);
  
  // Refresh background controls
  const bgColor = parseBgColorClass(el.classes);
  refreshColorPicker("bg-color", bgColor);
  
  // Refresh border controls
  const borders = parseBorderClasses(el.classes);
  refreshSelect("border-radius", borders.rounded);
  refreshSegmentedControl("border-width", borders.borderWidth);
  refreshColorPicker("border-color", borders.borderColor);
  
  // Refresh effects controls
  const effects = parseEffectsClasses(el.classes);
  refreshSlider("opacity", effects.opacity || "100");
  refreshSelect("shadow", effects.shadow);
}

/**
 * Refresh a segmented control to show the correct active value
 */
function refreshSegmentedControl(id, value) {
  const control = $panel(`[data-control="${id}"]`);
  if (!control) return;
  
  control.querySelectorAll(".seg-btn").forEach(btn => {
    btn.classList.toggle("active", btn.dataset.value === value);
  });
}

/**
 * Refresh a numeric input to show the correct value
 */
function refreshNumericInput(id, value) {
  const wrapper = $panel(`[data-control="${id}"]`);
  if (!wrapper) return;
  
  const input = wrapper.querySelector(".num-input");
  if (input) {
    input.value = value || "";
  }
  
  // Update preset active states
  wrapper.querySelectorAll(".num-preset").forEach(preset => {
    preset.classList.toggle("active", preset.dataset.value === value);
  });
}

/**
 * Refresh a select to show the correct value
 */
function refreshSelect(id, value) {
  const wrapper = $panel(`[data-control="${id}"]`);
  if (!wrapper) return;
  
  const select = wrapper.querySelector(".ctrl-select");
  if (select) {
    select.value = value || "";
  }
}

/**
 * Refresh a color picker to show the correct value
 */
function refreshColorPicker(id, value) {
  const picker = $panel(`[data-control="${id}"]`);
  if (!picker) return;
  
  const colorInput = picker.querySelector(".color-input");
  const textInput = picker.querySelector(".color-text");
  const swatches = picker.querySelectorAll(".color-swatch");
  
  // Update text input
  if (textInput) {
    textInput.value = value || "";
  }
  
  // Update color input if we have a valid hex
  if (colorInput && value) {
    const hex = TAILWIND_COLORS[value] || value;
    if (/^#[0-9a-fA-F]{6}$/.test(hex)) {
      colorInput.value = hex;
    }
  }
  
  // Update swatch active states
  swatches.forEach(swatch => {
    swatch.classList.toggle("active", swatch.dataset.color === value);
  });
}

/**
 * Refresh a slider to show the correct value
 */
function refreshSlider(id, value) {
  const slider = $panel(`[data-control="${id}"]`);
  if (!slider) return;
  
  const input = slider.querySelector(".slider-input");
  const valueDisplay = slider.querySelector(".slider-value");
  
  // Get values array for this slider
  const values = id === "opacity" ? OPACITY_VALUES : [];
  const idx = values.indexOf(value);
  
  if (input && idx >= 0) {
    input.value = idx;
    const percent = (idx / (values.length - 1)) * 100;
    input.style.setProperty("--percent", `${percent}%`);
  }
  
  if (valueDisplay) {
    valueDisplay.textContent = value;
  }
}

function bindCollapsibleSections() {
  $$panel(".design-section-header").forEach((header) => {
    header.addEventListener("click", () => {
      const section = header.closest(".design-section");
      const sectionName = header.dataset.section;
      const isExpanded = section.classList.toggle("expanded");
      setSectionCollapsed(sectionName, !isExpanded);
    });
  });
}

// LocalStorage for section collapse state
function getSectionCollapsed(section) {
  try {
    const stored = localStorage.getItem("rejoice-studio-sections");
    if (stored) {
      const state = JSON.parse(stored);
      // Only use stored value if it exists for this section
      if (state[section] !== undefined) {
        return state[section];
      }
    }
  } catch (e) {}
  // ALL sections collapsed by default - user expands what they need
  return true;
}

function setSectionCollapsed(section, collapsed) {
  try {
    const stored = localStorage.getItem("rejoice-studio-sections");
    const state = stored ? JSON.parse(stored) : {};
    state[section] = collapsed;
    localStorage.setItem("rejoice-studio-sections", JSON.stringify(state));
  } catch (e) {}
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
// Spacing Section
// =============================================================================

// Spacing mode: "unified" (p-4), "axis" (px-4 py-2), "individual" (pt-4 pr-2 pb-4 pl-2)
// Stored per-property-type (margin vs padding)
function getSpacingMode(type) {
  // type is "p" or "m"
  try {
    const stored = localStorage.getItem("rejoice-studio-spacing-mode");
    if (stored) {
      const modes = JSON.parse(stored);
      return modes[type] || "individual";
    }
  } catch (e) {}
  return "individual";
}

function setSpacingMode(type, mode) {
  try {
    const stored = localStorage.getItem("rejoice-studio-spacing-mode");
    const modes = stored ? JSON.parse(stored) : {};
    modes[type] = mode;
    localStorage.setItem("rejoice-studio-spacing-mode", JSON.stringify(modes));
  } catch (e) {}
}

/**
 * Detect the current spacing mode from classes
 * Returns "unified", "axis", or "individual"
 */
function detectSpacingMode(spacing, type) {
  const prefix = type; // "p" or "m"
  
  // Check what classes exist
  const hasUnified = spacing[prefix] !== null;
  const hasX = spacing[prefix + "x"] !== null;
  const hasY = spacing[prefix + "y"] !== null;
  const hasT = spacing[prefix + "t"] !== null;
  const hasR = spacing[prefix + "r"] !== null;
  const hasB = spacing[prefix + "b"] !== null;
  const hasL = spacing[prefix + "l"] !== null;
  
  // If only unified exists, it's unified mode
  if (hasUnified && !hasX && !hasY && !hasT && !hasR && !hasB && !hasL) {
    return "unified";
  }
  
  // If axis classes exist (and no individual overrides), it's axis mode
  if ((hasX || hasY || hasUnified) && !hasT && !hasR && !hasB && !hasL) {
    return "axis";
  }
  
  // Otherwise it's individual mode
  return "individual";
}

/**
 * Get the effective value for a spacing direction, considering cascade
 * Cascade order: individual (t/r/b/l) > axis (x/y) > unified (p/m)
 */
function getEffectiveSpacingValue(spacing, type, direction) {
  // direction is "t", "r", "b", "l", "x", "y", or "all"
  const prefix = type; // "p" or "m"
  
  if (direction === "all") {
    return spacing[prefix];
  }
  
  if (direction === "x") {
    return spacing[prefix + "x"] ?? spacing[prefix];
  }
  
  if (direction === "y") {
    return spacing[prefix + "y"] ?? spacing[prefix];
  }
  
  // Individual directions
  const axisMap = { t: "y", b: "y", l: "x", r: "x" };
  const axis = axisMap[direction];
  
  // Check individual first, then axis, then unified
  return spacing[prefix + direction] ?? spacing[prefix + axis] ?? spacing[prefix];
}

function renderSpacingSection(classes) {
  const spacing = parseSpacingClasses(classes);
  const isCollapsed = getSectionCollapsed("spacing");
  
  return `
    <div class="design-section ${isCollapsed ? "" : "expanded"}" data-section="spacing">
      <div class="design-section-header" data-section="spacing">
        <svg class="section-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2"/>
          <rect x="7" y="7" width="10" height="10" rx="1" stroke-dasharray="2 2"/>
        </svg>
        <span class="section-label">Spacing</span>
        <svg class="section-chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="6,9 12,15 18,9"/>
        </svg>
      </div>
      <div class="design-section-body">
        <div class="spacing-row">
          ${renderSpacingGroup(classes, spacing, "m", "M")}
          ${renderSpacingGroup(classes, spacing, "p", "P")}
        </div>
      </div>
    </div>
  `;
}

function renderSpacingGroup(classes, spacing, type, label) {
  const mode = detectSpacingMode(spacing, type);
  const colorClass = type === "m" ? "margin" : "padding";
  
  // Get effective values based on cascade
  const allVal = getEffectiveSpacingValue(spacing, type, "all");
  const xVal = getEffectiveSpacingValue(spacing, type, "x");
  const yVal = getEffectiveSpacingValue(spacing, type, "y");
  const tVal = getEffectiveSpacingValue(spacing, type, "t");
  const rVal = getEffectiveSpacingValue(spacing, type, "r");
  const bVal = getEffectiveSpacingValue(spacing, type, "b");
  const lVal = getEffectiveSpacingValue(spacing, type, "l");
  
  return `
    <div class="spacing-group" data-spacing-type="${type}">
      <div class="spacing-group-header">
        <span class="spacing-group-label ${colorClass}">${label}</span>
        <div class="spacing-mode-toggle" data-type="${type}">
          <button class="mode-btn ${mode === 'unified' ? 'active' : ''}" data-mode="unified" title="All sides equal">
            <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor">
              <rect x="3" y="3" width="18" height="18" rx="2"/>
            </svg>
          </button>
          <button class="mode-btn ${mode === 'axis' ? 'active' : ''}" data-mode="axis" title="Horizontal & Vertical">
            <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor">
              <rect x="3" y="10" width="18" height="4" rx="1"/>
              <rect x="10" y="3" width="4" height="18" rx="1"/>
            </svg>
          </button>
          <button class="mode-btn ${mode === 'individual' ? 'active' : ''}" data-mode="individual" title="Individual sides">
            <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor">
              <rect x="3" y="3" width="18" height="3" rx="1"/>
              <rect x="3" y="18" width="18" height="3" rx="1"/>
              <rect x="3" y="3" width="3" height="18" rx="1"/>
              <rect x="18" y="3" width="3" height="18" rx="1"/>
            </svg>
          </button>
        </div>
      </div>
      
      <div class="spacing-inputs" data-mode="${mode}">
        ${mode === 'unified' ? `
          <div class="spacing-unified">
            ${renderSpacingInput(type, "all", allVal, "")}
          </div>
        ` : mode === 'axis' ? `
          <div class="spacing-axis">
            ${renderSpacingInput(type + "x", "x", xVal, "↔")}
            ${renderSpacingInput(type + "y", "y", yVal, "↕")}
          </div>
        ` : `
          <div class="spacing-individual">
            ${renderSpacingInput(type + "t", "t", tVal, "↑")}
            ${renderSpacingInput(type + "r", "r", rVal, "→")}
            ${renderSpacingInput(type + "b", "b", bVal, "↓")}
            ${renderSpacingInput(type + "l", "l", lVal, "←")}
          </div>
        `}
      </div>
    </div>
  `;
}

function renderSpacingInput(prop, direction, value, icon) {
  const displayValue = value !== null ? value : "–";
  const type = prop.charAt(0); // "p" or "m"
  const colorClass = type === "m" ? "margin" : "padding";
  
  return `
    <div class="spacing-input ${colorClass}" data-prop="${prop}" data-direction="${direction}">
      <span class="spacing-input-icon">${icon}</span>
      <div class="spacing-input-value" data-value="${value || ""}">${displayValue}</div>
    </div>
  `;
}

function bindSpacingControls() {
  // Mode toggle buttons
  $$panel(".spacing-mode-toggle").forEach(toggle => {
    const type = toggle.dataset.type;
    
    toggle.querySelectorAll(".mode-btn").forEach(btn => {
      btn.addEventListener("click", () => {
        const mode = btn.dataset.mode;
        setSpacingMode(type, mode);
        
        // Convert classes to new mode
        convertSpacingToMode(type, mode);
        
        // Re-render
        refreshSpacingControls();
      });
    });
  });
  
  // Spacing input interactions
  $$panel(".spacing-input").forEach((input) => {
    const valueEl = input.querySelector(".spacing-input-value");
    if (!valueEl) return;
    
    let isDragging = false;
    let startX = 0;
    let startValue = null;
    let startIdx = 0;
    
    // Drag to adjust
    input.addEventListener("mousedown", (e) => {
      if (e.target.tagName === "INPUT") return;
      e.preventDefault();
      isDragging = true;
      startX = e.clientX;
      startValue = valueEl.dataset.value || "0";
      startIdx = SPACING_VALUES.indexOf(startValue);
      if (startIdx === -1) startIdx = 0;
      
      input.classList.add("dragging");
      document.body.style.cursor = "ew-resize";
      document.body.style.userSelect = "none";
    });
    
    const onMouseMove = (e) => {
      if (!isDragging) return;
      
      const deltaX = e.clientX - startX;
      const steps = Math.round(deltaX / 12);
      const newIdx = Math.max(0, Math.min(SPACING_VALUES.length - 1, startIdx + steps));
      const newValue = SPACING_VALUES[newIdx];
      
      valueEl.textContent = newValue;
      valueEl.dataset.value = newValue;
      
      const prop = input.dataset.prop;
      previewSpacingChange(prop, newValue);
    };
    
    const onMouseUp = () => {
      if (!isDragging) return;
      isDragging = false;
      
      input.classList.remove("dragging");
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
      
      const prop = input.dataset.prop;
      const newValue = valueEl.dataset.value;
      if (newValue !== startValue) {
        updateSpacingValue(prop, newValue);
      }
    };
    
    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);
    
    // Double-click to edit
    input.addEventListener("dblclick", (e) => {
      e.preventDefault();
      e.stopPropagation();
      const currentValue = valueEl.dataset.value || "";
      
      const editInput = document.createElement("input");
      editInput.type = "text";
      editInput.className = "spacing-edit-input";
      editInput.value = currentValue === "–" ? "" : currentValue;
      editInput.placeholder = "0";
      
      valueEl.style.display = "none";
      input.querySelector(".spacing-input-icon").style.display = "none";
      input.appendChild(editInput);
      editInput.focus();
      editInput.select();
      
      const finishEdit = () => {
        const newValue = editInput.value.trim() || null;
        const prop = input.dataset.prop;
        updateSpacingValue(prop, newValue);
      };
      
      editInput.addEventListener("blur", finishEdit);
      editInput.addEventListener("keydown", (e) => {
        if (e.key === "Enter") {
          e.preventDefault();
          finishEdit();
        }
        if (e.key === "Escape") {
          e.preventDefault();
          refreshSpacingControls();
        }
      });
    });
    
    // Scroll wheel
    input.addEventListener("wheel", (e) => {
      e.preventDefault();
      const prop = input.dataset.prop;
      const currentValue = valueEl.dataset.value || "0";
      const dir = e.deltaY < 0 ? 1 : -1;
      const newValue = stepSpacingValue(currentValue, dir);
      updateSpacingValue(prop, newValue);
    });
  });
}

/**
 * Convert spacing classes to a new mode
 * e.g., from "pt-4 pr-4 pb-4 pl-4" to "p-4"
 */
function convertSpacingToMode(type, newMode) {
  const el = State.selectedElement;
  if (!el) return;
  
  const spacing = parseSpacingClasses(el.classes);
  
  // Get current effective values
  const tVal = getEffectiveSpacingValue(spacing, type, "t");
  const rVal = getEffectiveSpacingValue(spacing, type, "r");
  const bVal = getEffectiveSpacingValue(spacing, type, "b");
  const lVal = getEffectiveSpacingValue(spacing, type, "l");
  
  // Remove all existing spacing classes for this type
  let classes = el.classes;
  classes = updateSpacingClass(classes, type, null);
  classes = updateSpacingClass(classes, type + "x", null);
  classes = updateSpacingClass(classes, type + "y", null);
  classes = updateSpacingClass(classes, type + "t", null);
  classes = updateSpacingClass(classes, type + "r", null);
  classes = updateSpacingClass(classes, type + "b", null);
  classes = updateSpacingClass(classes, type + "l", null);
  
  // Add new classes based on mode
  if (newMode === "unified") {
    // Use the most common value, or top value as fallback
    const values = [tVal, rVal, bVal, lVal].filter(v => v !== null);
    const commonValue = values.length > 0 ? values[0] : null;
    if (commonValue) {
      classes = updateSpacingClass(classes, type, commonValue);
    }
  } else if (newMode === "axis") {
    // X = left/right, Y = top/bottom
    const xVal = rVal || lVal;
    const yVal = tVal || bVal;
    if (xVal) classes = updateSpacingClass(classes, type + "x", xVal);
    if (yVal) classes = updateSpacingClass(classes, type + "y", yVal);
  } else {
    // Individual
    if (tVal) classes = updateSpacingClass(classes, type + "t", tVal);
    if (rVal) classes = updateSpacingClass(classes, type + "r", rVal);
    if (bVal) classes = updateSpacingClass(classes, type + "b", bVal);
    if (lVal) classes = updateSpacingClass(classes, type + "l", lVal);
  }
  
  // Update
  el.classes = classes;
  const classesInput = $panel("#classes-input");
  if (classesInput) classesInput.value = classes;
  
  send({ type: "preview-classes", path: el.path, classes: classes });
  
  const applyFooter = $panel("#apply-footer");
  if (applyFooter) {
    applyFooter.classList.toggle("hidden", el.classes === el.savedClasses);
  }
}

function previewSpacingChange(prop, value) {
  const el = State.selectedElement;
  if (!el) return;
  
  const newClasses = updateSpacingClass(el.classes, prop, value);
  send({ type: "preview-classes", path: el.path, classes: newClasses });
}

function updateSpacingValue(prop, value) {
  const el = State.selectedElement;
  if (!el) return;
  
  // Update the classes
  el.classes = updateSpacingClass(el.classes, prop, value);
  
  // Update the textarea
  const classesInput = $panel("#classes-input");
  if (classesInput) {
    classesInput.value = el.classes;
  }
  
  // Preview the change
  send({ type: "preview-classes", path: el.path, classes: el.classes });
  
  // Update apply footer visibility
  const applyFooter = $panel("#apply-footer");
  if (applyFooter) {
    applyFooter.classList.toggle("hidden", el.classes === el.savedClasses);
  }
  
  // Re-render spacing section to update display
  refreshSpacingControls();
}

// =============================================================================
// Layout Section
// =============================================================================

function renderLayoutSection(classes) {
  const layout = parseLayoutClasses(classes);
  const isCollapsed = getSectionCollapsed("layout");
  
  return `
    <div class="design-section ${isCollapsed ? "" : "expanded"}" data-section="layout">
      <div class="design-section-header" data-section="layout">
        <svg class="section-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2"/>
          <path d="M3 9h18M9 21V9"/>
        </svg>
        <span class="section-label">Layout</span>
        <svg class="section-chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="6,9 12,15 18,9"/>
        </svg>
      </div>
      <div class="design-section-body">
        <div class="control-row">
          <label class="control-label-inline">Display</label>
          ${renderSegmentedControl("display", [
            { value: "block", label: "Block" },
            { value: "flex", label: "Flex" },
            { value: "grid", label: "Grid" },
            { value: "hidden", label: "None" },
          ], layout.display, "display")}
        </div>
        
        <div class="control-row flex-controls ${layout.display === 'flex' ? '' : 'hidden'}">
          <label class="control-label-inline">Direction</label>
          ${renderSegmentedControl("flex-direction", [
            { value: "row", icon: "→" },
            { value: "row-reverse", icon: "←" },
            { value: "col", icon: "↓" },
            { value: "col-reverse", icon: "↑" },
          ], layout.flexDirection, "flex-direction")}
        </div>
        
        <div class="control-row flex-controls ${layout.display === 'flex' || layout.display === 'grid' ? '' : 'hidden'}">
          <label class="control-label-inline">Justify</label>
          ${renderSegmentedControl("justify", [
            { value: "start", icon: "⊢" },
            { value: "center", icon: "⊡" },
            { value: "end", icon: "⊣" },
            { value: "between", icon: "⊢⊣" },
          ], layout.justifyContent, "justify")}
        </div>
        
        <div class="control-row flex-controls ${layout.display === 'flex' || layout.display === 'grid' ? '' : 'hidden'}">
          <label class="control-label-inline">Align</label>
          ${renderSegmentedControl("align", [
            { value: "start", icon: "⊤" },
            { value: "center", icon: "⊡" },
            { value: "end", icon: "⊥" },
            { value: "stretch", icon: "↕" },
          ], layout.alignItems, "items")}
        </div>
        
        <div class="control-row flex-controls ${layout.display === 'flex' || layout.display === 'grid' ? '' : 'hidden'}">
          <label class="control-label-inline">Gap</label>
          ${renderNumericInput("gap", layout.gap, "gap", "0", ["0", "1", "2", "4", "6", "8"])}
        </div>
      </div>
    </div>
  `;
}

// =============================================================================
// Size Section
// =============================================================================

function renderSizeSection(classes) {
  const size = parseSizeClasses(classes);
  const isCollapsed = getSectionCollapsed("size");
  
  return `
    <div class="design-section ${isCollapsed ? "" : "expanded"}" data-section="size">
      <div class="design-section-header" data-section="size">
        <svg class="section-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 3H3v18h18V3z"/>
          <path d="M9 3v18M3 15h18"/>
        </svg>
        <span class="section-label">Size</span>
        <svg class="section-chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="6,9 12,15 18,9"/>
        </svg>
      </div>
      <div class="design-section-body">
        <div class="size-grid">
          <div class="size-row">
            <label class="size-label">W</label>
            ${renderNumericInput("width", size.w, "w", "auto", ["auto", "full", "screen", "fit"])}
          </div>
          <div class="size-row">
            <label class="size-label">H</label>
            ${renderNumericInput("height", size.h, "h", "auto", ["auto", "full", "screen", "fit"])}
          </div>
        </div>
        
        <div class="size-constraints">
          <div class="constraint-row">
            <span class="constraint-label">Min</span>
            <div class="constraint-inputs">
              ${renderNumericInput("min-w", size.minW, "min-w", "–")}
              ${renderNumericInput("min-h", size.minH, "min-h", "–")}
            </div>
          </div>
          <div class="constraint-row">
            <span class="constraint-label">Max</span>
            <div class="constraint-inputs">
              ${renderNumericInput("max-w", size.maxW, "max-w", "–")}
              ${renderNumericInput("max-h", size.maxH, "max-h", "–")}
            </div>
          </div>
        </div>
      </div>
    </div>
  `;
}

// =============================================================================
// Position Section
// =============================================================================

function renderPositionSection(classes) {
  const pos = parsePositionClasses(classes);
  const isCollapsed = getSectionCollapsed("position");
  const showInsetControls = pos.position && pos.position !== "static";
  
  return `
    <div class="design-section ${isCollapsed ? "" : "expanded"}" data-section="position">
      <div class="design-section-header" data-section="position">
        <svg class="section-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2"/>
          <circle cx="12" cy="12" r="2"/>
          <path d="M12 3v4M12 17v4M3 12h4M17 12h4"/>
        </svg>
        <span class="section-label">Position</span>
        <svg class="section-chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="6,9 12,15 18,9"/>
        </svg>
      </div>
      <div class="design-section-body">
        <div class="control-row">
          <label class="control-label-inline">Type</label>
          ${renderSegmentedControl("position", [
            { value: "static", label: "Static" },
            { value: "relative", label: "Rel" },
            { value: "absolute", label: "Abs" },
            { value: "fixed", label: "Fixed" },
            { value: "sticky", label: "Sticky" },
          ], pos.position || "static", "position")}
        </div>
        
        <div class="position-insets ${showInsetControls ? '' : 'hidden'}">
          <div class="control-row">
            <label class="control-label-inline">Inset</label>
            ${renderNumericInput("inset", pos.inset, "inset", "–", ["0", "auto"])}
          </div>
          
          <div class="inset-grid">
            <div class="inset-row">
              <label class="inset-label">T</label>
              ${renderNumericInput("top", pos.top, "top", "–", ["0", "auto"])}
            </div>
            <div class="inset-row">
              <label class="inset-label">R</label>
              ${renderNumericInput("right", pos.right, "right", "–", ["0", "auto"])}
            </div>
            <div class="inset-row">
              <label class="inset-label">B</label>
              ${renderNumericInput("bottom", pos.bottom, "bottom", "–", ["0", "auto"])}
            </div>
            <div class="inset-row">
              <label class="inset-label">L</label>
              ${renderNumericInput("left", pos.left, "left", "–", ["0", "auto"])}
            </div>
          </div>
          
          <div class="control-row">
            <label class="control-label-inline">Z-Index</label>
            ${renderNumericInput("z-index", pos.zIndex, "z", "–", ["0", "10", "20", "50"])}
          </div>
        </div>
      </div>
    </div>
  `;
}

// =============================================================================
// Overflow Section
// =============================================================================

function renderOverflowSection(classes) {
  const overflow = parseOverflowClasses(classes);
  const isCollapsed = getSectionCollapsed("overflow");
  
  return `
    <div class="design-section ${isCollapsed ? "" : "expanded"}" data-section="overflow">
      <div class="design-section-header" data-section="overflow">
        <svg class="section-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2"/>
          <path d="M3 15h18"/>
          <path d="M12 15v6"/>
        </svg>
        <span class="section-label">Overflow</span>
        <svg class="section-chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="6,9 12,15 18,9"/>
        </svg>
      </div>
      <div class="design-section-body">
        <div class="control-row">
          <label class="control-label-inline">All</label>
          ${renderSegmentedControl("overflow", [
            { value: "visible", label: "Vis" },
            { value: "hidden", label: "Hide" },
            { value: "auto", label: "Auto" },
            { value: "scroll", label: "Scroll" },
          ], overflow.overflow, "overflow")}
        </div>
        
        <div class="control-row">
          <label class="control-label-inline">X</label>
          ${renderSegmentedControl("overflow-x", [
            { value: "visible", label: "Vis" },
            { value: "hidden", label: "Hide" },
            { value: "auto", label: "Auto" },
            { value: "scroll", label: "Scroll" },
          ], overflow.overflowX, "overflow-x")}
        </div>
        
        <div class="control-row">
          <label class="control-label-inline">Y</label>
          ${renderSegmentedControl("overflow-y", [
            { value: "visible", label: "Vis" },
            { value: "hidden", label: "Hide" },
            { value: "auto", label: "Auto" },
            { value: "scroll", label: "Scroll" },
          ], overflow.overflowY, "overflow-y")}
        </div>
      </div>
    </div>
  `;
}

// =============================================================================
// Typography Section
// =============================================================================

function renderTypographySection(classes) {
  const typo = parseTypographyClasses(classes);
  const isCollapsed = getSectionCollapsed("typography");
  
  return `
    <div class="design-section ${isCollapsed ? "" : "expanded"}" data-section="typography">
      <div class="design-section-header" data-section="typography">
        <svg class="section-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M4 7V4h16v3"/>
          <path d="M9 20h6"/>
          <path d="M12 4v16"/>
        </svg>
        <span class="section-label">Typography</span>
        <svg class="section-chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="6,9 12,15 18,9"/>
        </svg>
      </div>
      <div class="design-section-body">
        <div class="control-row">
          <label class="control-label-inline">Size</label>
          ${renderSelect("font-size", FONT_SIZE_VALUES, typo.fontSize, "text", true)}
        </div>
        
        <div class="control-row">
          <label class="control-label-inline">Weight</label>
          ${renderSelect("font-weight", FONT_WEIGHT_VALUES, typo.fontWeight, "font")}
        </div>
        
        <div class="control-row">
          <label class="control-label-inline">Align</label>
          ${renderSegmentedControl("text-align", [
            { value: "left", icon: "⊢" },
            { value: "center", icon: "≡" },
            { value: "right", icon: "⊣" },
          ], typo.textAlign, "text-align")}
        </div>
        
        <div class="control-row">
          <label class="control-label-inline">Color</label>
          ${renderColorPicker("text-color", typo.textColor, "text-color")}
        </div>
      </div>
    </div>
  `;
}

// =============================================================================
// Background Section
// =============================================================================

function renderBackgroundSection(classes) {
  const bgColor = parseBgColorClass(classes);
  const isCollapsed = getSectionCollapsed("background");
  
  return `
    <div class="design-section ${isCollapsed ? "" : "expanded"}" data-section="background">
      <div class="design-section-header" data-section="background">
        <svg class="section-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2"/>
          <circle cx="8.5" cy="8.5" r="1.5"/>
          <path d="M21 15l-5-5L5 21"/>
        </svg>
        <span class="section-label">Background</span>
        <svg class="section-chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="6,9 12,15 18,9"/>
        </svg>
      </div>
      <div class="design-section-body">
        <div class="control-row">
          <label class="control-label-inline">Color</label>
          ${renderColorPicker("bg-color", bgColor, "bg")}
        </div>
      </div>
    </div>
  `;
}

// =============================================================================
// Borders Section
// =============================================================================

function renderBordersSection(classes) {
  const borders = parseBorderClasses(classes);
  const isCollapsed = getSectionCollapsed("borders");
  
  return `
    <div class="design-section ${isCollapsed ? "" : "expanded"}" data-section="borders">
      <div class="design-section-header" data-section="borders">
        <svg class="section-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="3"/>
        </svg>
        <span class="section-label">Borders</span>
        <svg class="section-chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="6,9 12,15 18,9"/>
        </svg>
      </div>
      <div class="design-section-body">
        <div class="control-row">
          <label class="control-label-inline">Radius</label>
          ${renderSelect("border-radius", RADIUS_VALUES.map(v => ({ value: v, label: v || "default" })), borders.rounded, "rounded")}
        </div>
        
        <div class="control-row">
          <label class="control-label-inline">Width</label>
          ${renderSegmentedControl("border-width", [
            { value: "0", label: "0" },
            { value: "", label: "1" },
            { value: "2", label: "2" },
            { value: "4", label: "4" },
            { value: "8", label: "8" },
          ], borders.borderWidth, "border")}
        </div>
        
        <div class="control-row">
          <label class="control-label-inline">Color</label>
          ${renderColorPicker("border-color", borders.borderColor, "border-color", [
            "transparent", "black", "white",
            "slate-200", "slate-300", "slate-400",
            "gray-200", "gray-300", "gray-400",
            "red-500", "blue-500", "green-500"
          ])}
        </div>
      </div>
    </div>
  `;
}

// =============================================================================
// Effects Section
// =============================================================================

function renderEffectsSection(classes) {
  const effects = parseEffectsClasses(classes);
  const isCollapsed = getSectionCollapsed("effects");
  
  return `
    <div class="design-section ${isCollapsed ? "" : "expanded"}" data-section="effects">
      <div class="design-section-header" data-section="effects">
        <svg class="section-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 2a15.3 15.3 0 014 10 15.3 15.3 0 01-4 10"/>
        </svg>
        <span class="section-label">Effects</span>
        <svg class="section-chevron" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <polyline points="6,9 12,15 18,9"/>
        </svg>
      </div>
      <div class="design-section-body">
        <div class="control-row">
          <label class="control-label-inline">Opacity</label>
          ${renderSlider("opacity", effects.opacity || "100", "opacity", OPACITY_VALUES)}
        </div>
        
        <div class="control-row">
          <label class="control-label-inline">Shadow</label>
          ${renderSelect("shadow", SHADOW_VALUES.map(v => ({ value: v, label: v || "default" })), effects.shadow, "shadow")}
        </div>
      </div>
    </div>
  `;
}

// =============================================================================
// Unified Design Control Binding
// =============================================================================

function bindDesignControls() {
  // Bind spacing controls (existing)
  bindSpacingControls();
  
  // Bind segmented controls
  $$panel(".segmented-control").forEach(control => {
    const prop = control.dataset.prop;
    
    control.querySelectorAll(".seg-btn").forEach(btn => {
      btn.addEventListener("click", () => {
        const value = btn.dataset.value;
        updateDesignProperty(prop, value);
        
        // Update active state
        control.querySelectorAll(".seg-btn").forEach(b => b.classList.remove("active"));
        btn.classList.add("active");
        
        // Show/hide flex controls when display changes
        if (prop === "display") {
          const flexControls = $$panel(".flex-controls");
          flexControls.forEach(fc => {
            fc.classList.toggle("hidden", value !== "flex" && value !== "grid");
          });
        }
      });
    });
  });
  
  // Bind select controls
  $$panel(".ctrl-select").forEach(select => {
    const wrapper = select.closest(".select-wrapper");
    const prop = wrapper?.dataset.prop;
    const customInput = wrapper?.querySelector(".select-custom");
    
    select.addEventListener("change", () => {
      const value = select.value;
      updateDesignProperty(prop, value || null);
      
      // Toggle custom input visibility
      if (customInput) {
        const isCustom = value === "" && select.dataset.allowCustom === "true";
        customInput.style.display = isCustom ? "block" : "none";
      }
    });
    
    // Handle custom input
    if (customInput) {
      customInput.addEventListener("input", debounce(() => {
        updateDesignProperty(prop, customInput.value || null);
      }, 150));
    }
  });
  
  // Bind numeric inputs
  $$panel(".numeric-input").forEach(wrapper => {
    const prop = wrapper.dataset.prop;
    const input = wrapper.querySelector(".num-input");
    const presets = wrapper.querySelectorAll(".num-preset");
    
    if (input) {
      input.addEventListener("input", debounce(() => {
        updateDesignProperty(prop, input.value || null);
      }, 150));
    }
    
    presets.forEach(preset => {
      preset.addEventListener("click", () => {
        const value = preset.dataset.value;
        if (input) input.value = value;
        updateDesignProperty(prop, value);
        
        // Update active states
        presets.forEach(p => p.classList.remove("active"));
        preset.classList.add("active");
      });
    });
  });
  
  // Bind color pickers
  $$panel(".color-picker").forEach(picker => {
    const prop = picker.dataset.prop;
    const colorInput = picker.querySelector(".color-input");
    const textInput = picker.querySelector(".color-text");
    const swatches = picker.querySelectorAll(".color-swatch");
    
    // Color input change
    if (colorInput) {
      colorInput.addEventListener("input", () => {
        const hex = colorInput.value;
        if (textInput) textInput.value = hex;
        // Convert hex to Tailwind color if possible, otherwise use arbitrary value
        const twColor = findTailwindColor(hex);
        updateDesignProperty(prop, twColor || `[${hex}]`);
      });
    }
    
    // Text input change
    if (textInput) {
      textInput.addEventListener("input", debounce(() => {
        const value = textInput.value.trim();
        updateDesignProperty(prop, value || null);
        
        // Update color input if it's a valid hex
        if (colorInput && /^#[0-9a-fA-F]{6}$/.test(value)) {
          colorInput.value = value;
        }
      }, 150));
    }
    
    // Swatch clicks
    swatches.forEach(swatch => {
      swatch.addEventListener("click", () => {
        const color = swatch.dataset.color;
        const hex = TAILWIND_COLORS[color] || color;
        
        if (textInput) textInput.value = color;
        if (colorInput && /^#[0-9a-fA-F]{6}$/.test(hex)) {
          colorInput.value = hex;
        }
        
        updateDesignProperty(prop, color);
        
        // Update active states
        swatches.forEach(s => s.classList.remove("active"));
        swatch.classList.add("active");
      });
    });
  });
  
  // Bind sliders
  $$panel(".slider-control").forEach(slider => {
    const prop = slider.dataset.prop;
    const input = slider.querySelector(".slider-input");
    const valueDisplay = slider.querySelector(".slider-value");
    
    if (input) {
      // Get the values array based on prop
      const values = prop === "opacity" ? OPACITY_VALUES : [];
      
      input.addEventListener("input", () => {
        const idx = parseInt(input.value);
        const value = values[idx];
        if (valueDisplay) valueDisplay.textContent = value;
        
        // Update CSS variable for track fill
        const percent = (idx / (values.length - 1)) * 100;
        input.style.setProperty("--percent", `${percent}%`);
        
        updateDesignProperty(prop, value);
      });
    }
  });
}

/**
 * Update a design property and sync to classes
 */
function updateDesignProperty(prop, value) {
  const el = State.selectedElement;
  if (!el) return;
  
  // Update classes based on property type
  switch (prop) {
    case "display":
      el.classes = updateDisplayClass(el.classes, value);
      break;
    case "flex-direction":
      el.classes = updateFlexDirectionClass(el.classes, value);
      break;
    case "position":
      el.classes = updatePositionClass(el.classes, value);
      // Show/hide inset controls
      const positionInsets = $$panel(".position-insets");
      positionInsets.forEach(pi => {
        pi.classList.toggle("hidden", !value || value === "static");
      });
      break;
    case "overflow":
      el.classes = updateOverflowClass(el.classes, null, value);
      break;
    case "overflow-x":
      el.classes = updateOverflowClass(el.classes, "x", value);
      break;
    case "overflow-y":
      el.classes = updateOverflowClass(el.classes, "y", value);
      break;
    case "text-align":
      el.classes = updateTextAlignClass(el.classes, value);
      break;
    case "text-color":
      el.classes = updateTextColorClass(el.classes, value);
      break;
    case "border-color":
      el.classes = updatePropertyClass(el.classes, "border", value);
      break;
    case "rounded":
      el.classes = updatePropertyClass(el.classes, "rounded", value);
      break;
    case "shadow":
      el.classes = updatePropertyClass(el.classes, "shadow", value);
      break;
    default:
      // Generic property update
      el.classes = updatePropertyClass(el.classes, prop, value);
  }
  
  // Update textarea
  const classesInput = $panel("#classes-input");
  if (classesInput) {
    classesInput.value = el.classes;
  }
  
  // Preview
  send({ type: "preview-classes", path: el.path, classes: el.classes });
  
  // Update apply footer
  const applyFooter = $panel("#apply-footer");
  if (applyFooter) {
    applyFooter.classList.toggle("hidden", el.classes === el.savedClasses);
  }
}

/**
 * Find the Tailwind color name for a hex value
 */
function findTailwindColor(hex) {
  const normalized = hex.toLowerCase();
  for (const [name, value] of Object.entries(TAILWIND_COLORS)) {
    if (value.toLowerCase() === normalized) {
      return name;
    }
  }
  return null;
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
    const [registryRes, enumsRes] = await Promise.all([
      fetch("/__studio/registry"),
      fetch("/__studio/enums"),
    ]);
    State.components = (await registryRes.json()).components || [];
    State.enumVariants = (await enumsRes.json()) || {};
    renderComponents();
  } catch (e) {
    console.error("Failed to fetch components:", e);
  }
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
  
  // Disable select mode and button when isolated
  if (State.selectMode) toggleSelect();
  $panel("#select-btn").disabled = true;

  // Load the component preview (this triggers enum registration on the server)
  loadComponentPreview();

  // Re-render inspect panel to show isolation controls
  renderInspect();
  
  // After a brief delay, re-fetch enums (they get registered when preview is rendered)
  // and re-render to show dropdowns for any enum props
  setTimeout(async () => {
    try {
      const res = await fetch("/__studio/enums");
      const newEnums = await res.json();
      // Only re-render if we got new enum types
      if (Object.keys(newEnums).length > Object.keys(State.enumVariants).length) {
        State.enumVariants = newEnums;
        renderInspect();
      }
    } catch (e) {}
  }, 500);
}

function disableIsolation() {
  State.isolatedComponent = null;
  $("#studio").classList.remove("isolated");
  $panel("#select-btn").disabled = false;
  State.previewIframe.src = "";
  renderInspect();
}

// Called from Components tab - creates a synthetic selection and enables isolation
function isolateComponent(name) {
  const meta = State.components.find((c) => c.name === name);
  if (!meta) {
    toast(`Component "${name}" not found in registry`, "error");
    return;
  }
  
  // Create a synthetic selected element for the component
  State.selectedElement = {
    tagName: "div",
    classes: "",
    id: null,
    componentName: name,
    isComponentRoot: true,
    sourceLocation: `${meta.file}:${meta.line}:${meta.column}`,
    path: null,
  };
  
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
  } else if (State.enumVariants[ty]) {
    // Enum type with known variants - use dropdown
    const variants = State.enumVariants[ty];
    const selectedValue = currentValue || variants[0] || "";
    inputHtml = `
      <select class="prop-select" data-prop="${name}">
        ${variants.map(v => `<option value="${v}" ${v === selectedValue ? 'selected' : ''}>${v}</option>`).join('')}
      </select>
    `;
  } else {
    // For unknown types, use text input
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

  // Select dropdowns (for enums)
  $$panel(".prop-select").forEach((select) => {
    select.addEventListener("change", () => {
      updateProp(select.dataset.prop, select.value);
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

// =============================================================================
// Utility Functions
// =============================================================================

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

/* Breakpoint-constrained canvas */
#studio.breakpoint-active #stage {
  display: flex;
  justify-content: center;
}

#studio.breakpoint-active #canvas {
  transition: max-width 0.3s ease;
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
  user-select: none;
  -webkit-user-select: none;
  cursor: default;
  ${CSS_VARS}
}

* { box-sizing: border-box; }

/* Allow text selection in inputs and textareas */
input, textarea, [contenteditable] {
  user-select: text;
  -webkit-user-select: text;
  cursor: text;
}

input[type="color"], input[type="range"], input[type="checkbox"] {
  cursor: default;
}

select {
  cursor: default;
}

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
.tool-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
  pointer-events: none;
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
   Context Bar (Breakpoint + State)
   ========================================================================== */

#context-bar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 14px;
  background: var(--bg2);
  border-bottom: 1px solid var(--border);
}

.context-group {
  display: flex;
  align-items: center;
  gap: 6px;
}

.context-label {
  font: 500 9px 'Space Grotesk', sans-serif;
  color: var(--text3);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.context-buttons {
  display: flex;
  gap: 2px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 2px;
}

.context-divider {
  width: 1px;
  height: 20px;
  background: var(--border);
}

.ctx-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  height: 22px;
  padding: 0 6px;
  background: none;
  border: none;
  border-radius: 4px;
  font: 500 10px 'JetBrains Mono', monospace;
  color: var(--text3);
  
  transition: all 0.1s ease;
}

.ctx-btn:hover {
  background: var(--bg3);
  color: var(--text2);
}

.ctx-btn.active {
  background: linear-gradient(135deg, rgba(240,171,252,0.2), rgba(129,140,248,0.2));
  color: var(--accent1);
}

#state-buttons .ctx-btn.active {
  background: linear-gradient(135deg, rgba(110,231,183,0.15), rgba(52,211,153,0.15));
  color: var(--green);
}

/* ==========================================================================
   Body
   ========================================================================== */

#body {
  flex: 1; 
  overflow-y: auto; 
  padding: 16px;
  position: relative;
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

.version-hint {
  font: 10px 'JetBrains Mono', monospace;
  color: var(--text3);
  text-align: center;
  margin-top: 8px;
  opacity: 0.6;
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
  min-height: 60px;
  padding: 10px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  font: 11px/1.5 'JetBrains Mono', monospace;
  color: var(--text);
  resize: vertical;
  transition: all 0.15s ease;
}

#classes-input::placeholder { color: var(--text3); }

#classes-input:focus {
  outline: none;
  border-color: var(--accent1);
  box-shadow: 0 0 0 2px rgba(240, 171, 252, 0.1);
}

/* Legacy apply button (now in footer) */
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
   Design Sections
   ========================================================================== */

.design-sections {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.design-section {
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  overflow: hidden;
}

.design-section-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 14px;
  
  transition: background 0.1s ease;
  border-bottom: 1px solid transparent;
}

.design-section.expanded .design-section-header {
  border-bottom-color: var(--border);
}

.design-section-header:hover {
  background: var(--bg3);
}

.section-icon {
  opacity: 0.5;
  flex-shrink: 0;
}

.design-section.expanded .section-icon {
  opacity: 0.8;
  color: var(--accent1);
}

.section-label {
  flex: 1;
  font: 600 11px 'Space Grotesk', sans-serif;
  color: var(--text2);
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

.design-section.expanded .section-label {
  color: var(--text);
}

.section-target {
  font-weight: 400;
  color: var(--text3);
  margin-left: 4px;
  text-transform: none;
  letter-spacing: normal;
}

.section-chevron {
  opacity: 0.4;
  transition: transform 0.15s ease;
}

.design-section.expanded .section-chevron {
  transform: rotate(180deg);
  opacity: 0.6;
}

.design-section-body {
  display: none;
  padding: 14px;
}

.design-section.expanded .design-section-body {
  display: block;
}

/* ==========================================================================
   Spacing Controls
   ========================================================================== */

.spacing-row {
  display: flex;
  gap: 16px;
}

.spacing-group {
  flex: 1;
}

.spacing-group-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.spacing-group-label {
  font: 600 10px 'JetBrains Mono', monospace;
}

.spacing-group-label.margin {
  color: var(--accent1);
}

.spacing-group-label.padding {
  color: var(--green);
}

.spacing-mode-toggle {
  display: flex;
  gap: 1px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 2px;
}

.mode-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 16px;
  padding: 0;
  background: none;
  border: none;
  border-radius: 2px;
  color: var(--text3);
  transition: all 0.1s ease;
}

.mode-btn:hover {
  color: var(--text2);
}

.mode-btn.active {
  background: var(--bg4);
  color: var(--text);
}

/* Unified mode - single input */
.spacing-unified .spacing-input {
  width: 100%;
}

/* Axis mode - two inputs stacked */
.spacing-axis {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

/* Individual mode - 2x2 grid */
.spacing-individual {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4px;
}

/* Spacing input */
.spacing-input {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 5px 8px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 4px;
  cursor: ew-resize;
  transition: all 0.1s ease;
}

.spacing-input:hover {
  border-color: var(--border-light);
  background: var(--bg3);
}

.spacing-input.dragging {
  background: var(--bg4);
}

.spacing-input.margin {
  border-color: rgba(240, 171, 252, 0.25);
}

.spacing-input.margin:hover,
.spacing-input.margin.dragging {
  border-color: var(--accent1);
}

.spacing-input.padding {
  border-color: rgba(110, 231, 183, 0.25);
}

.spacing-input.padding:hover,
.spacing-input.padding.dragging {
  border-color: var(--green);
}

.spacing-input-icon {
  font: 400 9px 'Space Grotesk', sans-serif;
  color: var(--text3);
  opacity: 0.6;
}

.spacing-input-value {
  font: 500 10px 'JetBrains Mono', monospace;
  color: var(--text);
  min-width: 16px;
  text-align: center;
}

.spacing-edit-input {
  width: 30px;
  padding: 0;
  background: none;
  border: none;
  font: 500 11px 'JetBrains Mono', monospace;
  color: var(--text);
  text-align: center;
  outline: none;
}

/* ==========================================================================
   Apply Footer
   ========================================================================== */

.apply-footer {
  padding: 12px 16px;
  background: var(--bg2);
  border-top: 1px solid var(--border);
}

.apply-footer.hidden {
  display: none;
}

.apply-footer button {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  padding: 12px 16px;
  background: linear-gradient(135deg, var(--accent1), var(--accent2));
  border: none;
  border-radius: var(--radius-sm);
  font: 600 13px 'Space Grotesk', sans-serif;
  color: var(--void);
  
  transition: all 0.15s ease;
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);
}

.apply-footer button:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 20px -5px var(--accent-glow);
}

.apply-footer button:active {
  transform: translateY(0) scale(0.98);
}

.apply-footer kbd {
  padding: 2px 6px;
  background: rgba(0,0,0,0.2);
  border-radius: 3px;
  font: 500 10px 'JetBrains Mono', monospace;
  opacity: 0.8;
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

/* Enum select dropdown */
.prop-select {
  width: 100%;
  padding: 8px 10px;
  background: var(--bg2);
  border: 1px solid var(--border);
  border-radius: 6px;
  font: 11px 'JetBrains Mono', monospace;
  color: var(--text);
  
  transition: all 0.15s ease;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 10px center;
  padding-right: 30px;
}

.prop-select:focus {
  outline: none;
  border-color: var(--green);
  box-shadow: 0 0 0 3px rgba(110, 231, 183, 0.1);
}

.prop-select option {
  background: var(--bg2);
  color: var(--text);
  padding: 8px;
}

/* Boolean toggle */
.prop-toggle {
  position: relative;
  display: inline-flex;
  width: 44px;
  height: 24px;
  
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

/* ==========================================================================
   Control Row Layout
   ========================================================================== */

.control-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 14px;
}

.control-row:last-child {
  margin-bottom: 0;
}

.control-row.hidden {
  display: none;
}

.control-label-inline {
  min-width: 56px;
  font: 500 10px 'Space Grotesk', sans-serif;
  color: var(--text3);
  text-transform: uppercase;
  letter-spacing: 0.03em;
  flex-shrink: 0;
}

/* ==========================================================================
   Segmented Control
   ========================================================================== */

.segmented-control {
  display: flex;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 2px;
  flex: 1;
}

.seg-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 5px 6px;
  background: none;
  border: none;
  border-radius: 4px;
  font: 500 10px 'Space Grotesk', sans-serif;
  color: var(--text3);
  
  transition: all 0.1s ease;
}

.seg-btn:hover {
  background: var(--bg3);
  color: var(--text2);
}

.seg-btn.active {
  background: linear-gradient(135deg, rgba(240,171,252,0.2), rgba(129,140,248,0.2));
  color: var(--accent1);
}

.seg-icon {
  font-size: 12px;
  line-height: 1;
}

.seg-label {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ==========================================================================
   Select Control
   ========================================================================== */

.select-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.ctrl-select {
  width: 100%;
  padding: 6px 28px 6px 10px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  font: 500 11px 'JetBrains Mono', monospace;
  color: var(--text);
  
  transition: all 0.1s ease;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='10' viewBox='0 0 24 24' fill='none' stroke='%236a6a78' stroke-width='2.5'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 8px center;
}

.ctrl-select:hover {
  border-color: var(--border-light);
}

.ctrl-select:focus {
  outline: none;
  border-color: var(--accent1);
}

.ctrl-select option {
  background: var(--bg2);
  color: var(--text);
}

.select-custom {
  padding: 6px 10px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  font: 500 11px 'JetBrains Mono', monospace;
  color: var(--text);
}

.select-custom:focus {
  outline: none;
  border-color: var(--accent1);
}

/* ==========================================================================
   Numeric Input
   ========================================================================== */

.numeric-input {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.num-input {
  width: 100%;
  padding: 6px 10px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  font: 500 11px 'JetBrains Mono', monospace;
  color: var(--text);
  text-align: center;
  transition: all 0.1s ease;
}

.num-input::placeholder {
  color: var(--text3);
}

.num-input:hover {
  border-color: var(--border-light);
}

.num-input:focus {
  outline: none;
  border-color: var(--accent1);
}

.num-presets {
  display: flex;
  gap: 2px;
}

.num-preset {
  flex: 1;
  padding: 3px 4px;
  background: none;
  border: 1px solid transparent;
  border-radius: 4px;
  font: 500 9px 'JetBrains Mono', monospace;
  color: var(--text3);
  
  transition: all 0.1s ease;
}

.num-preset:hover {
  background: var(--bg3);
  color: var(--text2);
}

.num-preset.active {
  background: var(--bg3);
  border-color: var(--border);
  color: var(--text);
}

/* ==========================================================================
   Color Picker
   ========================================================================== */

.color-picker {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.color-preview-row {
  display: flex;
  align-items: center;
  gap: 6px;
}

.color-input {
  width: 32px;
  height: 32px;
  padding: 0;
  background: none;
  border: 1px solid var(--border);
  border-radius: 6px;
  
  overflow: hidden;
  flex-shrink: 0;
}

.color-input::-webkit-color-swatch-wrapper {
  padding: 3px;
}

.color-input::-webkit-color-swatch {
  border: none;
  border-radius: 3px;
}

.color-text {
  flex: 1;
  padding: 6px 10px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  font: 500 11px 'JetBrains Mono', monospace;
  color: var(--text);
}

.color-text::placeholder {
  color: var(--text3);
}

.color-text:focus {
  outline: none;
  border-color: var(--accent1);
}

.color-swatches {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.color-swatch {
  width: 20px;
  height: 20px;
  padding: 0;
  border: 1px solid var(--border);
  border-radius: 4px;
  
  transition: all 0.1s ease;
  font-size: 10px;
  color: var(--text3);
  display: flex;
  align-items: center;
  justify-content: center;
}

.color-swatch:hover {
  transform: scale(1.15);
  border-color: var(--border-light);
}

.color-swatch.active {
  box-shadow: 0 0 0 2px var(--accent1);
}

.color-swatch.transparent {
  background: linear-gradient(45deg, 
    var(--bg3) 25%, transparent 25%,
    transparent 75%, var(--bg3) 75%
  );
  background-size: 8px 8px;
}

/* ==========================================================================
   Slider Control
   ========================================================================== */

.slider-control {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 10px;
}

.slider-input {
  flex: 1;
  height: 4px;
  background: var(--bg4);
  border-radius: 2px;
  appearance: none;
  
  position: relative;
}

.slider-input::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  width: var(--percent, 50%);
  background: linear-gradient(90deg, var(--accent1), var(--accent2));
  border-radius: 2px;
  pointer-events: none;
}

.slider-input::-webkit-slider-thumb {
  appearance: none;
  width: 14px;
  height: 14px;
  background: var(--text);
  border: 2px solid var(--bg);
  border-radius: 50%;
  cursor: grab;
  position: relative;
  z-index: 1;
  box-shadow: 0 1px 4px rgba(0,0,0,0.3);
}

.slider-input::-webkit-slider-thumb:active {
  cursor: grabbing;
}

.slider-value {
  min-width: 28px;
  font: 500 11px 'JetBrains Mono', monospace;
  color: var(--text2);
  text-align: right;
}

/* ==========================================================================
   Size Section Specific
   ========================================================================== */

.size-grid {
  display: flex;
  gap: 10px;
  margin-bottom: 12px;
}

.size-row {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 6px;
}

.size-label {
  font: 600 10px 'JetBrains Mono', monospace;
  color: var(--text3);
  width: 14px;
  text-align: center;
}

.size-constraints {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding-top: 10px;
  border-top: 1px solid var(--border);
}

.constraint-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.constraint-label {
  font: 500 9px 'Space Grotesk', sans-serif;
  color: var(--text3);
  text-transform: uppercase;
  letter-spacing: 0.03em;
  width: 28px;
}

.constraint-inputs {
  display: flex;
  gap: 8px;
  flex: 1;
}

.constraint-inputs .numeric-input {
  flex: 1;
}

.constraint-inputs .num-input {
  padding: 5px 8px;
  font-size: 10px;
}

/* ==========================================================================
   Position Insets
   ========================================================================== */

.position-insets {
  margin-top: 10px;
}

.inset-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
  margin-top: 8px;
  padding: 10px;
  background: var(--bg);
  border-radius: 6px;
  border: 1px solid var(--border);
}

.inset-row {
  display: flex;
  align-items: center;
  gap: 6px;
}

.inset-label {
  font: 600 10px 'JetBrains Mono', monospace;
  color: var(--text3);
  width: 12px;
  text-align: center;
}

.inset-row .numeric-input {
  flex: 1;
}

.inset-row .num-input {
  padding: 5px 8px;
  font-size: 10px;
}
  `;
}

// =============================================================================
// Start
// =============================================================================

init();
