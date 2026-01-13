# Rejoice Studio - Planning Document

> A visual development environment built into the Rejoice framework, enabling AI-powered UI editing directly from the browser.

## Philosophy

- **Opt-in**: The framework remains excellent standalone. Studio is additive, not required.
- **Full power, minimal friction**: Rich features that don't get in the way.
- **Agent-first**: Built around the idea that AI agents do most of the work, with humans guiding.
- **No bloat in production**: Studio is completely stripped from release builds.

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────┐
│  Dev Overlay UI (Web Components + Shadow DOM)   │
│  - Element inspector/selector                   │
│  - Property editor (Tailwind classes, etc.)     │
│  - Agent chat interface                         │
│  - Component browser/storybook                  │
│  - DOM tree view                                │
├─────────────────────────────────────────────────┤
│  Dev Server Extensions (Rust)                   │
│  - Source mapping (element → file:line)         │
│  - Component registry/metadata                  │
│  - WebSocket API for edits                      │
├─────────────────────────────────────────────────┤
│  Agent Backend (OpenCode SDK)                   │
│  - Receives prompts + context                   │
│  - Makes file edits                             │
│  - Streams responses back                       │
└─────────────────────────────────────────────────┘
```

---

## Activation

**Command:** `rejoice dev --studio`

- Without `--studio`: Normal dev server, no Studio code injected
- With `--studio`: Studio overlay injected, WebSocket endpoints activated

---

## UI Design

### Entry Point

**Minimal indicator approach:**
- Small floating button/pill in corner of viewport
- Click to open full Studio panel
- Unobtrusive during normal development
- Keyboard shortcut to toggle: `Cmd + .` (macOS) / `Ctrl + .` (Windows/Linux)

### Main Panel Layout

When opened, the Studio panel provides:

1. **Element Inspector** - Click to select elements in the page
2. **DOM Tree View** - Hierarchical view of components/elements (like devtools)
3. **Properties Panel** - For selected element:
   - Source location (file:line, clickable to open in editor)
   - Tailwind classes (editable with autocomplete)
   - Component props (if it's a component)
   - Computed styles (read-only)
   - Parent chain breadcrumb
4. **Agent Chat** - Prompt input with streaming responses
5. **Component Browser** - List of all registered components with storybook-like playground

### Element Selection

**Smart mode:**
- Default: Select component boundaries
- Modifier key (e.g., Alt/Option): Select any element
- DOM tree view as fallback for tricky z-index situations

### Agent Prompt Scope

- **Global chat**: For page-wide or app-wide prompts ("add a footer", "create a new page")
- **Selection-scoped**: When element selected, prompts default to that scope
- **Agent infers scope** from prompt content
- Optional explicit scope selector: `[Component] [Page] [App]`

---

## Component System

### The `#[component]` Macro

Transforms a function into a component struct with builder pattern API.

**Definition:**
```rust
#[component]
pub fn Button(
    /// The button text (required - no default)
    label: &str,
    /// Size variant
    #[prop(default = ButtonSize::Medium)]
    size: ButtonSize,
    /// Whether the button is disabled
    #[prop(default = false)]
    disabled: bool,
) -> Markup {
    html! {
        button 
            class=(format!("btn {}", size.class()))
            disabled[disabled]
        {
            (label)
        }
    }
}
```

**Usage (builder pattern):**
```rust
html! {
    div class="flex gap-4" {
        (Button::new("Cancel"))
        (Button::new("Submit").size(ButtonSize::Large))
        (Button::new("Delete").size(ButtonSize::Small).disabled(true))
    }
}
```

No `.render()` call needed - components implement Maud's `Render` trait, so `(Component::new(...))` works directly in `html!`.

**What the macro generates:**

```rust
pub struct Button<'a> {
    label: &'a str,
    size: ButtonSize,
    disabled: bool,
}

impl<'a> Button<'a> {
    // Required props go in new()
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            size: ButtonSize::Medium,  // default
            disabled: false,           // default
        }
    }
    
    // Optional props get builder methods
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

// Maud integration - no explicit .render() needed
impl Render for Button<'_> {
    fn render(&self) -> Markup {
        html! {
            button 
                class=(format!("btn {}", self.size.class()))
                disabled[self.disabled]
            {
                (self.label)
            }
        }
    }
}
```

**What the macro extracts (for Studio):**
- Component name
- Prop names and types
- Required vs optional (has default or not)
- Default values (via `#[prop(default = ...)]`)
- Doc comments (for UI hints/descriptions)
- Source file and line number

**What the macro generates (dev mode only):**
- Registration call to global component registry
- Source location data attributes on rendered elements (`data-component`, `data-source`)

### Component Registry

Dev-mode-only global registry containing:
- All registered components
- Their prop metadata (names, types, defaults, docs)
- Source locations
- TODO: Possibly example values for storybook?

### Auto-generated Storybook

For each `#[component]`:
- Render in isolation
- Auto-generate prop controls based on type:
  - `bool` → checkbox
  - `&str` / `String` → text input
  - Enum → dropdown
  - Numbers → number input (or slider?)
  - Colors (if detectable) → color picker
- Show all prop combinations? Or just interactive playground?

---

## Source Mapping

### Debug Attributes

In dev mode, inject `data-source="file:line:col"` into elements:

```html
<div data-source="src/routes/index.rs:42:5" class="card">
  <h2 data-source="src/routes/index.rs:43:9">Title</h2>
</div>
```

**Implementation approach:**
- Modify the `html!` macro (or wrap it) to inject attributes in dev mode
- Use `cfg!(debug_assertions)` or a feature flag
- Component macro adds `data-component="ComponentName"` as well

### Source Location Click-to-Open

Deferred for now - not in initial implementation.

---

## Live Editing

### Tailwind Class Editing

1. User edits classes in Studio panel
2. Optimistic update: Apply immediately in browser
3. Send change to dev server via WebSocket
4. Dev server writes to source file
5. HMR triggers (user already sees change, so seamless)

### Component Prop Editing

Same flow as Tailwind classes:
1. Edit prop in Studio panel
2. Optimistic update
3. Write back to source
4. HMR

### Agent Edits

1. User enters prompt in chat
2. Context gathered: selected element, source file, component tree, available components
3. Request sent to agent backend (OpenCode SDK)
4. Agent streams response to UI
5. File edits shown in real-time
6. HMR reloads affected code

---

## Agent Integration

### OpenCode SDK

Reference: https://opencode.ai/docs/sdk/

TODO: Research SDK capabilities:
- How to initialize
- How to send prompts with context
- How to stream responses
- How to handle file edits

### Context Provided to Agent

When prompt is sent:
- Selected element's source code
- Full file content
- Component hierarchy/tree
- List of available components (with their prop signatures)
- Tailwind config/theme (if present)
- Screenshot of selected element? (TODO: investigate feasibility)

### Streaming UI

- Show agent's thinking/response as it streams
- Show file edits as diffs
- Allow user to interrupt/cancel
- Allow follow-up prompts

---

## WebSocket Protocol

### Endpoints

- `/__studio` - Main Studio WebSocket (when `--studio` flag used)

### Message Types (Draft)

**Client → Server:**
```typescript
{ type: "select", selector: string }
{ type: "edit_classes", source: string, oldClasses: string, newClasses: string }
{ type: "edit_prop", source: string, component: string, prop: string, value: any }
{ type: "agent_prompt", prompt: string, context: AgentContext }
{ type: "open_file", file: string, line: number }
```

**Server → Client:**
```typescript
{ type: "component_registry", components: ComponentMeta[] }
{ type: "edit_applied", success: boolean, error?: string }
{ type: "agent_stream", chunk: string }
{ type: "agent_complete", result: AgentResult }
{ type: "file_changed", file: string }
```

---

## File Structure

```
src/
├── bin/
│   └── commands/
│       └── dev.rs              # --studio flag handling
├── studio/                     # Studio module
│   ├── mod.rs                  # Module exports
│   ├── overlay.rs              # Serves overlay assets
│   ├── websocket.rs            # WebSocket handler
│   ├── registry.rs             # Component registry
│   ├── agent.rs                # OpenCode SDK integration
│   └── protocol.rs             # Message types
├── component.rs                # #[component] macro
├── assets/
│   ├── live_reload.js          # Existing HMR script
│   └── studio/                 # Studio UI assets
│       ├── overlay.js          # Web Component UI
│       └── overlay.css         # Isolated styles
```

---

## Build Considerations

### Dev Mode Only

All Studio code must be:
- Behind `cfg!(debug_assertions)` or a feature flag
- Not included in release builds
- No runtime cost when not using `--studio`

### Feature Flag Option

```toml
[features]
default = []
studio = []  # Enables Studio support
```

Or just use debug/release distinction?

TODO: Decide on feature flag vs debug_assertions

---

## Bun Migration

Switching from npm/node to Bun. **Bun is required** - no fallback to npm.

### Changes Required

1. **`init.rs`** - Generate for Bun:
   - No `package-lock.json`, Bun uses `bun.lockb`
   - Update generated scripts if any reference npm

2. **`dev.rs`** - Use `bun` commands:
   - `bun install` instead of `npm install`
   - `bun run` instead of `npm run` / `npx`
   - Check for bun availability on startup, error with install instructions if missing

3. **`build.rs`** - Same changes as dev.rs

---

## Implementation Phases

All features ship in v1 - no phased rollout. Organized by implementation order/dependencies:

### Layer 1: Core Infrastructure
- [ ] Bun migration (update init.rs, dev.rs, build.rs)
- [ ] `rejoice-macros` crate setup in workspace
- [ ] `#[component]` proc macro (struct + builder + Render impl)
- [ ] `#[derive(PropEnum)]` proc macro for enum variants
- [ ] `#[prop(default = ...)]` and `#[prop(default)]` attribute support
- [ ] Component registry (dev-mode global registry of component metadata)
- [ ] Source location injection (`data-source`, `data-component` attributes)

### Layer 2: Dev Server & Protocol
- [ ] `--studio` flag in `rejoice dev`
- [ ] Studio WebSocket endpoint (`/__studio`)
- [ ] Protocol messages (registry sync, edits, agent prompts)
- [ ] File write-back from Studio edits
- [ ] In-memory undo/redo (file backups)

### Layer 3: Studio UI (Web Components)
- [ ] Studio overlay shell (Web Component + Shadow DOM)
- [ ] Floating toggle button (minimal indicator)
- [ ] Element selector (click to select, smart mode with modifier key)
- [ ] DOM tree view (hierarchical, for tricky selections)
- [ ] Properties panel:
  - [ ] Source location display
  - [ ] Tailwind class editor with autocomplete
  - [ ] Component props editor (auto-generated controls)
  - [ ] Parent chain breadcrumb
- [ ] Component browser / storybook:
  - [ ] List all registered components
  - [ ] Render in isolation
  - [ ] Auto-generated prop controls by type
- [ ] Responsive viewport controls

### Layer 4: Agent Integration
- [ ] OpenCode SDK integration (Rust side)
- [ ] Agent chat UI in overlay
- [ ] Context gathering:
  - [ ] Selected element source code
  - [ ] Full file content
  - [ ] Component registry
  - [ ] Screenshot via html2canvas (element or viewport)
- [ ] Streaming response display
- [ ] Multi-file edit visualization
- [ ] Session history (conversation log)

---

## Decisions Made

1. **Component API**: Builder pattern with `Component::new(required).optional(value)` syntax
2. **Maud integration**: Components implement `Render` trait, no explicit `.render()` needed
3. **Editor integration**: Deferred - not in initial implementation
4. **Undo/redo**: In-memory file backups (not git-based), user handles git commits
5. **Agent error handling**: Show errors in Studio, let user prompt agent to fix (no auto-rollback)
6. **Enum props**: Custom `#[derive(PropEnum)]` macro for storybook dropdown generation
7. **Prop defaults**:
   - `#[prop(default = X)]` → Optional, uses X
   - `#[prop(default)]` → Optional, uses `Default::default()`
   - `Option<T>` without `#[prop]` → Optional, uses `None`
   - Everything else → Required (goes in `new()`)
8. **Children**: Explicit `children: Markup` prop with `.children(html!{...})` builder method
9. **Screenshots**: Use `getDisplayMedia` API for pixel-perfect screen capture (requires one-time permission prompt)
10. **Studio UI assets**: Pre-built and shipped with crate via `include_str!()`
11. **Component registry transport**: WebSocket message on connect
12. **Workspace structure**: Add `rejoice-macros` crate to existing workspace
13. **Keyboard shortcut**: `Cmd + .` (Ctrl + . on Windows/Linux) to toggle Studio panel
14. **Dev-only code**: Use `cfg!(debug_assertions)` - simple, automatic, no config needed
15. **Bun**: Required, no fallback to npm - error if not installed
16. **Multi-file edit UI**: Collapsed list with expandable diffs per file
17. **Error states**: Inline errors with retry options (WebSocket disconnect, file write fail, agent fail)
18. **Studio UI theme**: Dark mode only, polished aesthetic
19. **Onboarding**: None - discoverable by exploration

## Open Questions / TODOs

None - all major decisions made!

---

## Post-Implementation Documentation

- [ ] Guide on `#[component]` macro usage
- [ ] Guide on `#[derive(PropEnum)]` for enum props
- [ ] Guide on Studio features and workflow
- [ ] Update `LLM_DOCS.md` with component system and Studio context
- [ ] Update `AGENTS.md` with new crate structure and macros
- [ ] Update `README.md` with Studio feature overview

---

## References

- OpenCode SDK: https://opencode.ai/docs/sdk/
- Current HMR implementation: `src/assets/live_reload.js`, `src/bin/commands/dev.rs`
- Current build: `src/bin/commands/build.rs`
