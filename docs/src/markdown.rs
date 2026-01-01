use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};
use rejoice::{html as maud_html, Markup, PreEscaped};

/// Renders markdown to HTML Markup, with special handling for code blocks.
pub fn render_markdown(content: &str) -> Markup {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_HEADING_ATTRIBUTES
        | Options::ENABLE_TASKLISTS;

    let parser = Parser::new_ext(content, options);
    let parser = CodeBlockTransformer::new(parser);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    maud_html! {
        (PreEscaped(html_output))
    }
}

struct CodeBlockTransformer<'a, I: Iterator<Item = Event<'a>>> {
    inner: I,
    in_code_block: bool,
    current_lang: String,
    code_buffer: String,
}

impl<'a, I: Iterator<Item = Event<'a>>> CodeBlockTransformer<'a, I> {
    fn new(inner: I) -> Self {
        Self {
            inner,
            in_code_block: false,
            current_lang: String::new(),
            code_buffer: String::new(),
        }
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for CodeBlockTransformer<'a, I> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next()? {
                Event::Start(Tag::CodeBlock(kind)) => {
                    self.in_code_block = true;
                    self.code_buffer.clear();
                    self.current_lang = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                        pulldown_cmark::CodeBlockKind::Indented => String::new(),
                    };
                }
                Event::Text(text) if self.in_code_block => {
                    self.code_buffer.push_str(&text);
                }
                Event::End(TagEnd::CodeBlock) => {
                    self.in_code_block = false;
                    let lang = if self.current_lang.is_empty() {
                        "text"
                    } else {
                        &self.current_lang
                    };
                    let escaped_code =
                        html_escape::encode_double_quoted_attribute(&self.code_buffer);
                    let display_code = html_escape::encode_text(&self.code_buffer);

                    // Full code block with header, dots, and copy button
                    let html = format!(
                        r##"<div class="code-block my-6 rounded-2xl overflow-hidden border" style="border-color: var(--line);" data-lang="{lang}" data-code="{escaped_code}">
<div class="code-block-header relative flex items-center justify-between px-4 h-11" style="background: var(--surface-2); border-bottom: 1px solid var(--line);">
<div class="code-block-dots flex gap-2">
<span class="code-block-dot w-3 h-3 rounded-full transition-colors" style="background: var(--surface-4);"></span>
<span class="code-block-dot w-3 h-3 rounded-full transition-colors" style="background: var(--surface-4);"></span>
<span class="code-block-dot w-3 h-3 rounded-full transition-colors" style="background: var(--surface-4);"></span>
</div>
<button class="code-block-copy flex items-center justify-center w-8 h-8 rounded-lg transition-colors cursor-pointer hover:bg-white/5" style="color: var(--text-ghost);" type="button" aria-label="Copy code" onclick="window.__copyCode && window.__copyCode(this)">
<svg class="copy-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"></path></svg>
<svg class="check-icon hidden" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"></polyline></svg>
</button>
</div>
<pre class="p-5 overflow-x-auto"><code class="font-mono text-sm leading-relaxed">{display_code}</code></pre>
</div>"##,
                        lang = lang,
                        escaped_code = escaped_code,
                        display_code = display_code
                    );
                    return Some(Event::Html(html.into()));
                }
                event => return Some(event),
            }
        }
    }
}

/// Renders a code block with optional filename
pub fn code_block_with_filename(code: &str, lang: &str, filename: Option<&str>) -> Markup {
    let escaped_code = html_escape::encode_double_quoted_attribute(code);
    let display_code = html_escape::encode_text(code);

    maud_html! {
        div
            class="code-block my-6 rounded-2xl overflow-hidden border"
            style="border-color: var(--line);"
            data-lang=(lang)
            data-code=(PreEscaped(&escaped_code))
            data-filename=[filename]
        {
            // Header
            div class="code-block-header relative flex items-center justify-between px-4 h-11"
                style="background: var(--surface-2); border-bottom: 1px solid var(--line);" {
                // Traffic light dots
                div class="code-block-dots flex gap-2" {
                    span class="code-block-dot w-3 h-3 rounded-full transition-colors" style="background: var(--surface-4);" {}
                    span class="code-block-dot w-3 h-3 rounded-full transition-colors" style="background: var(--surface-4);" {}
                    span class="code-block-dot w-3 h-3 rounded-full transition-colors" style="background: var(--surface-4);" {}
                }
                // Filename
                @if let Some(name) = filename {
                    span class="absolute left-1/2 -translate-x-1/2 font-mono text-xs" style="color: var(--text-ghost);" { (name) }
                }
                // Copy button
                button
                    class="code-block-copy flex items-center justify-center w-8 h-8 rounded-lg transition-colors cursor-pointer hover:bg-white/5"
                    style="color: var(--text-ghost);"
                    type="button"
                    aria-label="Copy code"
                    onclick="window.__copyCode && window.__copyCode(this)"
                {
                    svg class="copy-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" {
                        rect x="9" y="9" width="13" height="13" rx="2" ry="2" {}
                        path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" {}
                    }
                    svg class="check-icon hidden" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" {
                        polyline points="20 6 9 17 4 12" {}
                    }
                }
            }
            // Code
            pre class="p-5 overflow-x-auto" {
                code class="font-mono text-sm leading-relaxed" {
                    (PreEscaped(&display_code))
                }
            }
        }
    }
}

/// Simple code block without header
#[allow(dead_code)]
pub fn code_block(code: &str, lang: &str) -> Markup {
    code_block_with_filename(code, lang, None)
}
