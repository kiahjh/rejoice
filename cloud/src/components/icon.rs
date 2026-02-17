use rejoice::{html, Markup};

pub fn github(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="currentColor"
        {
            path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" {}
        }
    }
}

pub fn plus(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            line x1="12" y1="5" x2="12" y2="19" {}
            line x1="5" y1="12" x2="19" y2="12" {}
        }
    }
}

pub fn external_link(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" {}
            polyline points="15 3 21 3 21 9" {}
            line x1="10" y1="14" x2="21" y2="3" {}
        }
    }
}

pub fn trash(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            polyline points="3 6 5 6 21 6" {}
            path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" {}
            line x1="10" y1="11" x2="10" y2="17" {}
            line x1="14" y1="11" x2="14" y2="17" {}
        }
    }
}

pub fn settings(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            circle cx="12" cy="12" r="3" {}
            path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" {}
        }
    }
}

pub fn rocket(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z" {}
            path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z" {}
            path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0" {}
            path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5" {}
        }
    }
}

pub fn database(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            ellipse cx="12" cy="5" rx="9" ry="3" {}
            path d="M3 5V19A9 3 0 0 0 21 19V5" {}
            path d="M3 12A9 3 0 0 0 21 12" {}
        }
    }
}

pub fn git_branch(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            line x1="6" y1="3" x2="6" y2="15" {}
            circle cx="18" cy="6" r="3" {}
            circle cx="6" cy="18" r="3" {}
            path d="M18 9a9 9 0 0 1-9 9" {}
        }
    }
}

pub fn git_commit(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            circle cx="12" cy="12" r="4" {}
            line x1="1.05" y1="12" x2="7" y2="12" {}
            line x1="17.01" y1="12" x2="22.96" y2="12" {}
        }
    }
}

pub fn check_circle(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" {}
            polyline points="22 4 12 14.01 9 11.01" {}
        }
    }
}

pub fn x_circle(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            circle cx="12" cy="12" r="10" {}
            line x1="15" y1="9" x2="9" y2="15" {}
            line x1="9" y1="9" x2="15" y2="15" {}
        }
    }
}

pub fn clock(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            circle cx="12" cy="12" r="10" {}
            polyline points="12 6 12 12 16 14" {}
        }
    }
}

pub fn globe(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            circle cx="12" cy="12" r="10" {}
            line x1="2" y1="12" x2="22" y2="12" {}
            path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" {}
        }
    }
}

pub fn folder(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" {}
        }
    }
}

pub fn key(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            path d="m21 2-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0 3 3L22 7l-3-3m-3.5 3.5L19 4" {}
        }
    }
}

pub fn terminal(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            polyline points="4 17 10 11 4 5" {}
            line x1="12" y1="19" x2="20" y2="19" {}
        }
    }
}

pub fn arrow_right(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            line x1="5" y1="12" x2="19" y2="12" {}
            polyline points="12 5 19 12 12 19" {}
        }
    }
}

pub fn cloud(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            path d="M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z" {}
        }
    }
}

pub fn zap(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" {}
        }
    }
}

pub fn copy(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            rect x="9" y="9" width="13" height="13" rx="2" ry="2" {}
            path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" {}
        }
    }
}

pub fn refresh(size: u32) -> Markup {
    html! {
        svg
            width=(size)
            height=(size)
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        {
            polyline points="23 4 23 10 17 10" {}
            polyline points="1 20 1 14 7 14" {}
            path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" {}
        }
    }
}
