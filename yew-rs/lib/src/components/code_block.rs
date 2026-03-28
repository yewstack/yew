use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use yew::prelude::*;
use yew_site_proc::comp;

thread_local! {
    static SS: SyntaxSet = SyntaxSet::load_defaults_newlines();
}

fn strip_highlight_directives(code: &str) -> (String, Vec<usize>) {
    let mut cleaned = String::new();
    let mut highlighted = Vec::new();
    let mut output_line: usize = 0;
    let mut in_highlight_block = false;

    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed == "// highlight-next-line" || trimmed == "//highlight-next-line" {
            highlighted.push(output_line + 1);
            continue;
        }
        if trimmed == "// highlight-start" || trimmed == "//highlight-start" {
            in_highlight_block = true;
            continue;
        }
        if trimmed == "// highlight-end" || trimmed == "//highlight-end" {
            in_highlight_block = false;
            continue;
        }
        // Diff markers: lines starting with '-' are removed, '+' prefix is stripped
        if let Some(rest) = line.strip_prefix('-') {
            if rest.is_empty() || rest.starts_with(char::is_whitespace) {
                continue;
            }
        }
        let emit_line = if let Some(rest) = line.strip_prefix('+') {
            if rest.is_empty() || rest.starts_with(char::is_whitespace) {
                output_line += 1;
                highlighted.push(output_line);
                // Reconstruct original indentation: '+' replaced one space
                cleaned.push(' ');
                cleaned.push_str(rest);
                cleaned.push('\n');
                false
            } else {
                true
            }
        } else {
            true
        };
        if emit_line {
            output_line += 1;
            if in_highlight_block {
                highlighted.push(output_line);
            }
            cleaned.push_str(line);
            cleaned.push('\n');
        }
    }

    (cleaned, highlighted)
}

fn highlight_code(code: &str, language: &str) -> (String, String) {
    let (cleaned, highlighted_lines) = strip_highlight_directives(code);
    let html = SS.with(|ss| {
        let syntax = ss
            .find_syntax_by_token(language)
            .unwrap_or_else(|| ss.find_syntax_plain_text());
        let mut gen = ClassedHTMLGenerator::new_with_class_style_and_highlighted_lines(
            syntax,
            ss,
            ClassStyle::SpacedPrefixed { prefix: "sy-" },
            &highlighted_lines,
        );
        for line in LinesWithEndings::from(&cleaned) {
            let _ = gen.parse_html_for_line_which_includes_newline(line);
        }
        gen.finalize()
    });
    (cleaned, html)
}

#[comp]
pub fn CodeBlock(
    code: AttrValue,
    #[prop_or("rust".into())] language: AttrValue,
    #[prop_or_default] title: AttrValue,
) {
    #[allow(unused_variables)]
    let (cleaned_code, highlighted) = highlight_code(&code, &language);
    let highlighted_html = Html::from_html_unchecked(AttrValue::from(highlighted));

    #[cfg(feature = "csr")]
    let copy_button = {
        let (copied, onclick) = crate::use_clipboard(AttrValue::from(cleaned_code));
        let copy_opacity = if copied { "1" } else { "0" };
        let copy_color = if copied {
            "#00a400"
        } else {
            "var(--color-text-secondary)"
        };
        html! {
            <button
                class={css!(
                    position: absolute;
                    top: 0.5rem;
                    right: 0.5rem;
                    background: var(--color-bg-secondary);
                    border: 1px solid var(--color-border);
                    border-radius: 4px;
                    cursor: pointer;
                    padding: 0.25rem 0.375rem;
                    color: ${copy_color};
                    opacity: ${copy_opacity};
                    transition: opacity 0.2s, background 0.2s, color 0.2s;
                    display: flex;
                    align-items: center;
                    z-index: 1;
                    &:hover {
                        background: var(--color-border);
                        color: var(--color-text);
                        opacity: 1;
                    }
                )}
                {onclick}
                title={if copied { "Copied!" } else { "Copy" }}
                aria-label="Copy code to clipboard"
            >
                if copied {
                    <svg viewBox="0 0 24 24" width="18" height="18">
                        <path fill="currentColor" d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                    </svg>
                } else {
                    <svg viewBox="0 0 640 640" width="18" height="18">
                        <path fill="currentColor" d="M480 400L288 400C279.2 400 272 392.8 272 384L272 128C272 119.2 279.2 112 288 112L421.5 112C425.7 112 429.8 113.7 432.8 116.7L491.3 175.2C494.3 178.2 496 182.3 496 186.5L496 384C496 392.8 488.8 400 480 400zM288 448L480 448C515.3 448 544 419.3 544 384L544 186.5C544 169.5 537.3 153.2 525.3 141.2L466.7 82.7C454.7 70.7 438.5 64 421.5 64L288 64C252.7 64 224 92.7 224 128L224 384C224 419.3 252.7 448 288 448zM160 192C124.7 192 96 220.7 96 256L96 512C96 547.3 124.7 576 160 576L352 576C387.3 576 416 547.3 416 512L416 496L368 496L368 512C368 520.8 360.8 528 352 528L160 528C151.2 528 144 520.8 144 512L144 256C144 247.2 151.2 240 160 240L176 240L176 192L160 192z"/>
                    </svg>
                }
            </button>
        }
    };
    #[cfg(not(feature = "csr"))]
    let copy_button = html! {};

    html! {
        <div class={css!(
            margin-bottom: 1.5rem;
            border-radius: 8px;
            overflow: hidden;
            border: 1px solid var(--color-border);
            &:hover button { opacity: 1; }
        )}>
            if !title.is_empty() {
                <div class={css!(
                    background: var(--color-bg-secondary);
                    padding: 0.5rem 1rem;
                    font-size: 0.8125rem;
                    font-weight: 600;
                    border-bottom: 1px solid var(--color-border);
                    font-family: var(--font-mono);
                )}>{&title}</div>
            }
            <div class={css!(position: relative;)}>
                {copy_button}
                <pre class={css!(
                    margin: 0;
                    padding: 1rem;
                    overflow-x: auto;
                    background: var(--color-code-bg);
                    font-size: 0.875rem;
                    line-height: 1.5;
                )}>
                    <code class={css!(display: inline-grid; min-width: 100%;)}>{highlighted_html}</code>
                </pre>
            </div>
        </div>
    }
}
