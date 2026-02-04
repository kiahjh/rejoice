use rejoice::{html, Markup};

pub fn input(name: &str, placeholder: &str) -> Markup {
    html! {
        input
            type="text"
            name=(name)
            id=(name)
            placeholder=(placeholder)
            autocomplete="off"
            class="w-full h-10 px-3 text-sm \
                   bg-stone-900 border border-stone-800 rounded-lg \
                   text-stone-100 placeholder-stone-600 \
                   outline-none transition-colors \
                   focus:border-stone-600";
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
            class="w-full h-10 px-3 text-sm \
                   bg-stone-900 border border-stone-800 rounded-lg \
                   text-stone-100 placeholder-stone-600 \
                   outline-none transition-colors \
                   focus:border-stone-600";
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
            class="w-full h-10 px-3 text-sm font-mono \
                   bg-stone-900 border border-stone-800 rounded-lg \
                   text-stone-100 placeholder-stone-600 \
                   outline-none transition-colors \
                   focus:border-stone-600";
    }
}

pub fn label(for_id: &str, text: &str) -> Markup {
    html! {
        label
            for=(for_id)
            class="block text-sm font-medium text-stone-300 mb-1.5"
        {
            (text)
        }
    }
}

pub fn form_group(children: Markup) -> Markup {
    html! {
        div class="space-y-1.5" {
            (children)
        }
    }
}

pub fn form_divider() -> Markup {
    html! {
        div class="h-px bg-stone-800 my-6" {}
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
                            rounded border border-stone-600 bg-stone-900 \
                            transition-all duration-150 \
                            group-hover:border-stone-500"
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
            span class="text-sm text-stone-400 group-hover:text-stone-300 transition-colors" {
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
                            rounded-full bg-stone-700 \
                            transition-colors duration-200"
                {
                    // Toggle knob
                    span class="toggle-knob absolute top-0.5 left-0.5 w-4 h-4 \
                                rounded-full bg-white shadow-sm \
                                transition-transform duration-200" {}
                }
            }

            // Label text
            span class="text-sm text-stone-400 group-hover:text-stone-300 transition-colors" {
                (label_text)
            }
        }
    }
}
