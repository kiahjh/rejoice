use rejoice::{html, Markup};

pub fn input(name: &str, placeholder: &str) -> Markup {
    html! {
        input
            type="text"
            name=(name)
            id=(name)
            placeholder=(placeholder)
            autocomplete="off"
            class="w-full h-10 px-3.5 text-sm \
                   bg-[var(--bg-base)] border border-[var(--border-default)] rounded-lg \
                   text-[var(--text-primary)] placeholder-[var(--text-faint)] \
                   outline-none transition-all duration-150 \
                   hover:border-[var(--border-strong)] \
                   focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent-subtle)] \
                   input-glow";
    }
}

pub fn input_with_value(name: &str, placeholder: &str, value: &str) -> Markup {
    html! {
        input
            type="text"
            name=(name)
            id=(name)
            placeholder=(placeholder)
            value=(value)
            autocomplete="off"
            class="w-full h-10 px-3.5 text-sm \
                   bg-[var(--bg-base)] border border-[var(--border-default)] rounded-lg \
                   text-[var(--text-primary)] placeholder-[var(--text-faint)] \
                   outline-none transition-all duration-150 \
                   hover:border-[var(--border-strong)] \
                   focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent-subtle)] \
                   input-glow";
    }
}

pub fn input_password(name: &str, placeholder: &str) -> Markup {
    html! {
        input
            type="password"
            name=(name)
            id=(name)
            placeholder=(placeholder)
            autocomplete="off"
            class="w-full h-10 px-3.5 text-sm font-mono \
                   bg-[var(--bg-base)] border border-[var(--border-default)] rounded-lg \
                   text-[var(--text-primary)] placeholder-[var(--text-faint)] \
                   outline-none transition-all duration-150 \
                   hover:border-[var(--border-strong)] \
                   focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent-subtle)] \
                   input-glow";
    }
}

/// Large search input with icon
pub fn input_search(name: &str, placeholder: &str) -> Markup {
    html! {
        div class="relative" {
            // Search icon
            div class="absolute left-3.5 top-1/2 -translate-y-1/2 text-[var(--text-faint)] pointer-events-none" {
                svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                    circle cx="11" cy="11" r="8" {}
                    line x1="21" y1="21" x2="16.65" y2="16.65" {}
                }
            }
            input
                type="text"
                name=(name)
                id=(name)
                placeholder=(placeholder)
                autocomplete="off"
                class="w-full h-11 pl-10 pr-4 text-sm \
                       bg-[var(--bg-base)] border border-[var(--border-default)] rounded-xl \
                       text-[var(--text-primary)] placeholder-[var(--text-faint)] \
                       outline-none transition-all duration-150 \
                       hover:border-[var(--border-strong)] \
                       focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent-subtle)] \
                       input-glow";
        }
    }
}

pub fn label(for_id: &str, text: &str) -> Markup {
    html! {
        label
            for=(for_id)
            class="block text-sm font-medium text-[var(--text-secondary)] mb-2"
        {
            (text)
        }
    }
}

/// Label with helper text
pub fn label_with_hint(for_id: &str, text: &str, hint: &str) -> Markup {
    html! {
        label for=(for_id) class="block mb-2" {
            span class="text-sm font-medium text-[var(--text-secondary)]" { (text) }
            span class="ml-2 text-xs text-[var(--text-faint)]" { (hint) }
        }
    }
}

pub fn form_group(children: Markup) -> Markup {
    html! {
        div class="space-y-2" {
            (children)
        }
    }
}

pub fn form_divider() -> Markup {
    html! {
        div class="h-px bg-[var(--border-subtle)] my-6" {}
    }
}

/// Helper/error text below inputs
pub fn form_helper(text: &str) -> Markup {
    html! {
        p class="mt-1.5 text-xs text-[var(--text-muted)]" { (text) }
    }
}

pub fn form_error(text: &str) -> Markup {
    html! {
        p class="mt-1.5 text-xs text-red-400" { (text) }
    }
}

/// A beautiful custom checkbox with label.
/// Uses a hidden real checkbox for form submission with a styled visual overlay.
pub fn checkbox(name: &str, label_text: &str, checked: bool) -> Markup {
    html! {
        label class="inline-flex items-center gap-2.5 cursor-pointer select-none group" {
            // Container for checkbox + hidden input
            span class="relative" {
                // Hidden real checkbox for form submission
                input
                    type="checkbox"
                    name=(name)
                    id=(name)
                    checked[checked]
                    class="checkbox-input absolute opacity-0 w-4 h-4 cursor-pointer";

                // Custom checkbox visual
                span class="checkbox-box flex items-center justify-center w-4 h-4 \
                            rounded border border-[var(--border-strong)] bg-[var(--bg-base)] \
                            transition-all duration-150 \
                            group-hover:border-[var(--accent)]"
                {
                    // Checkmark icon (opacity controlled by CSS)
                    svg
                        class="w-2.5 h-2.5 text-white opacity-0 transition-opacity"
                        viewBox="0 0 12 10"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    {
                        polyline points="1.5 5 4.5 8 10.5 2" {}
                    }
                }
            }

            // Label text
            span class="text-sm text-[var(--text-muted)] group-hover:text-[var(--text-secondary)] transition-colors" {
                (label_text)
            }
        }
    }
}

/// A toggle switch - alternative to checkbox for on/off settings.
pub fn toggle(name: &str, label_text: &str, checked: bool) -> Markup {
    html! {
        label class="inline-flex items-center gap-3 cursor-pointer select-none group" {
            // Container for toggle
            span class="relative" {
                // Hidden real checkbox for form submission
                input
                    type="checkbox"
                    name=(name)
                    id=(name)
                    checked[checked]
                    class="toggle-input absolute opacity-0 w-9 h-5 cursor-pointer";

                // Toggle track
                span class="toggle-track block w-9 h-5 \
                            rounded-full bg-[var(--bg-surface)] border border-[var(--border-default)] \
                            transition-all duration-200 \
                            group-hover:border-[var(--border-strong)]"
                {
                    // Toggle knob
                    span class="toggle-knob absolute top-0.5 left-0.5 w-4 h-4 \
                                rounded-full bg-[var(--text-muted)] shadow-sm \
                                transition-all duration-200" {}
                }
            }

            // Label text
            span class="text-sm text-[var(--text-muted)] group-hover:text-[var(--text-secondary)] transition-colors" {
                (label_text)
            }
        }
    }
}

/// Select/dropdown input
pub fn select(name: &str, options: &[(&str, &str)], selected: Option<&str>) -> Markup {
    html! {
        div class="relative" {
            select
                name=(name)
                id=(name)
                class="w-full h-10 px-3.5 pr-10 text-sm appearance-none \
                       bg-[var(--bg-base)] border border-[var(--border-default)] rounded-lg \
                       text-[var(--text-primary)] \
                       outline-none transition-all duration-150 cursor-pointer \
                       hover:border-[var(--border-strong)] \
                       focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent-subtle)]"
            {
                @for (value, label) in options {
                    option value=(value) selected[selected == Some(*value)] { (label) }
                }
            }
            // Dropdown arrow
            div class="absolute right-3 top-1/2 -translate-y-1/2 text-[var(--text-muted)] pointer-events-none" {
                svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                    polyline points="6 9 12 15 18 9" {}
                }
            }
        }
    }
}

/// Textarea for longer content
pub fn textarea(name: &str, placeholder: &str, rows: u32) -> Markup {
    html! {
        textarea
            name=(name)
            id=(name)
            placeholder=(placeholder)
            rows=(rows)
            class="w-full px-3.5 py-3 text-sm \
                   bg-[var(--bg-base)] border border-[var(--border-default)] rounded-lg \
                   text-[var(--text-primary)] placeholder-[var(--text-faint)] \
                   outline-none transition-all duration-150 resize-y \
                   hover:border-[var(--border-strong)] \
                   focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent-subtle)] \
                   input-glow"
        {}
    }
}
