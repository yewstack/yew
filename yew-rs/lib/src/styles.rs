use stylist::yew::Global;
use stylist::{css, StyleSource};
use yew::prelude::*;

#[component]
pub fn GlobalStyles() -> Html {
    let syntect_light: StyleSource = SYNTECT_LIGHT_THEME
        .try_into()
        .expect("syntect light theme CSS");
    let syntect_dark: StyleSource = SYNTECT_DARK_THEME
        .try_into()
        .expect("syntect dark theme CSS");

    html! {
        <>
            <Global css={css!(r#"
                :root {
                    --color-primary: #00755a;
                    --color-primary-dark: #006951;
                    --color-primary-darker: #00634d;
                    --color-primary-darkest: #00523f;
                    --color-primary-light: #008163;
                    --color-primary-lighter: #008768;
                    --color-primary-lightest: #009875;

                    --color-bg: #ffffff;
                    --color-bg-secondary: #f6f8fa;
                    --color-text: #1c1e21;
                    --color-text-secondary: #606770;
                    --color-border: #dadde1;
                    --color-code-bg: #f6f8fa;
                    --color-hero-bg: #f9fafb;

                    --font-sans: system-ui, -apple-system, "Segoe UI", Roboto, Ubuntu, Cantarell, "Noto Sans", sans-serif;
                    --font-mono: "SFMono-Regular", Menlo, Consolas, "Liberation Mono", monospace;

                    --h1-font-size: 2rem;
                    --navbar-height: 3.75rem;
                    --sidebar-width: 300px;
                    --content-max-width: 800px;

                    --admonition-note-bg: #f0f0f0;
                    --admonition-tip-bg: #e6f6e6;
                    --admonition-info-bg: #e6f0ff;
                    --admonition-warning-bg: #fff8e6;
                    --admonition-danger-bg: #ffe6e6;
                    --admonition-caution-bg: #fff8e6;
                    --admonition-important-bg: #f3e8ff;

                    --sy-hl-bg: rgba(0, 100, 200, 0.08);

                    --theme-sun-display: none;
                    --theme-moon-display: none;
                    --theme-system-display: block;

                    --color-bg-offset: #ebedf0;
                    --color-primary-rgb: 0, 117, 90;
                }

                [data-theme="dark"] {
                    --color-primary: #28d2ad;
                    --color-primary-dark: #24bd9c;
                    --color-primary-darker: #22b393;
                    --color-primary-darkest: #1c9379;
                    --color-primary-light: #3ad9b7;
                    --color-primary-lighter: #44dbba;
                    --color-primary-lightest: #64e1c6;

                    --color-bg: #1b1b1d;
                    --color-bg-secondary: #242526;
                    --color-text: #e3e3e3;
                    --color-text-secondary: #b0b0b0;
                    --color-border: #3e4042;
                    --color-code-bg: #2d2d2d;
                    --color-hero-bg: #242526;

                    --admonition-note-bg: #2a2a2c;
                    --admonition-tip-bg: #1a2e1a;
                    --admonition-info-bg: #1a2133;
                    --admonition-warning-bg: #332b1a;
                    --admonition-danger-bg: #331a1a;
                    --admonition-caution-bg: #332b1a;
                    --admonition-important-bg: #2a1a33;

                    --sy-hl-bg: rgba(255, 255, 255, 0.07);

                    --theme-sun-display: none;
                    --theme-moon-display: block;
                    --theme-system-display: none;

                    --color-bg-offset: #3a3a3c;
                    --color-primary-rgb: 40, 210, 173;

                    color-scheme: dark;
                }

                [data-theme-choice="light"] {
                    --theme-sun-display: block;
                    --theme-moon-display: none;
                    --theme-system-display: none;
                }

                [data-theme-choice="dark"] {
                    --theme-sun-display: none;
                    --theme-moon-display: block;
                    --theme-system-display: none;
                }

                *, *::before, *::after {
                    box-sizing: border-box;
                    margin: 0;
                    padding: 0;
                }

                html {
                    font-size: 16px;
                    -webkit-font-smoothing: antialiased;
                    -moz-osx-font-smoothing: grayscale;
                }

                body {
                    font-family: var(--font-sans);
                    color: var(--color-text);
                    background: var(--color-bg);
                    line-height: 1.65;
                }

                h1, h2, h3, h4, h5, h6 {
                    line-height: 1.25;
                    margin-top: 1.5rem;
                    margin-bottom: 0.75rem;
                    font-weight: 700;
                }

                h1 { font-size: var(--h1-font-size); }
                h2 { font-size: 1.5rem; }
                h3 { font-size: 1.25rem; }
                h4 { font-size: 1rem; }

                p {
                    margin-bottom: 1rem;
                }

                ul, ol {
                    padding-left: 2rem;
                    margin-bottom: 1rem;
                }

                code {
                    font-family: var(--font-mono);
                    font-size: 0.875em;
                    background: var(--color-code-bg);
                    padding: 0.125rem 0.375rem;
                    border-radius: 4px;
                }

                pre code {
                    background: none;
                    padding: 0;
                }

                img {
                    max-width: 100%;
                    height: auto;
                }

                .themed-img-dark {
                    display: none;
                }

                [data-theme="dark"] .themed-img-light {
                    display: none;
                }

                [data-theme="dark"] .themed-img-dark {
                    display: block;
                }
            "#)} />
            <Global css={syntect_light} />
            <Global css={syntect_dark} />
        </>
    }
}

const SYNTECT_LIGHT_THEME: &str = r#"
.sy-code {
 color: #323232;
 background-color: #ffffff;
}

.sy-comment {
 color: #969896;
font-style: italic;
}
.sy-string {
 color: #183691;
}
.sy-regexp-operator {
 color: #a71d5d;
}
.sy-string.sy-regexp.sy-characterclass .sy-punctuation.sy-definition.sy-string.sy-begin, .sy-string.sy-regexp.sy-characterclass .sy-punctuation.sy-definition.sy-string.sy-end {
 color: #a71d5d;
}
.sy-constant.sy-numeric {
 color: #0086b3;
}
.sy-constant.sy-language {
 color: #0086b3;
}
.sy-constant.sy-character, .sy-constant.sy-other, .sy-variable.sy-other.sy-constant {
 color: #0086b3;
}
.sy-variable {
 color: #323232;
}
.sy-keyword {
 color: #a71d5d;
font-weight: bold;
}
.sy-bitwise-operator {
 color: #a71d5d;
font-weight: bold;
}
.sy-storage {
 color: #a71d5d;
font-weight: bold;
}
.sy-storage.sy-type {
 color: #a71d5d;
font-weight: bold;
}
.sy-entity.sy-name.sy-class {
 color: #0086b3;
}
.sy-entity.sy-other.sy-inherited-class {
 color: #0086b3;
}
.sy-entity.sy-name.sy-function {
 color: #795da3;
font-weight: bold;
}
.sy-variable.sy-parameter {
 color: #323232;
}
.sy-entity.sy-name.sy-tag {
 color: #63a35c;
}
.sy-entity.sy-other.sy-attribute-name {
 color: #795da3;
}
.sy-support.sy-function {
 color: #62a35c;
}
.sy-support.sy-constant {
 color: #0086b3;
}
.sy-support.sy-type, .sy-support.sy-class {
 color: #0086b3;
}
.sy-support.sy-other.sy-variable {
 color: #323232;
}
.sy-invalid, .sy-invalid.sy-illegal, .sy-invalid.sy-deprecated {
 color: #b52a1d;
 background-color: #f5f5f5;
font-weight: bold;
}
.sy-entity.sy-name.sy-filename.sy-find-in-files {
 color: #323232;
font-weight: bold;
}
.sy-constant.sy-numeric.sy-line-number.sy-find-in-files, .sy-constant.sy-numeric.sy-line-number.sy-match.sy-find-in-files {
 color: #b3b3b3;
}
.sy-meta.sy-diff.sy-header {
 color: #969896;
 background-color: #ffffff;
font-style: italic;
}
.sy-meta.sy-diff.sy-header .sy-punctuation.sy-definition.sy-from-file.sy-diff {
 color: #bd2c00;
 background-color: #ffecec;
font-weight: bold;
font-style: italic;
}
.sy-meta.sy-diff.sy-header .sy-punctuation.sy-definition.sy-to-file.sy-diff {
 color: #55a532;
 background-color: #eaffea;
font-weight: bold;
font-style: italic;
}
.sy-meta.sy-diff.sy-range {
 color: #969896;
font-weight: bold;
font-style: italic;
}
.sy-markup.sy-deleted {
 background-color: #ffecec;
}
.sy-markup.sy-deleted .sy-punctuation.sy-definition.sy-inserted {
 color: #bd2c00;
font-weight: bold;
}
.sy-markup.sy-inserted {
 background-color: #eaffea;
}
.sy-markup.sy-inserted .sy-punctuation.sy-definition.sy-inserted {
 color: #55a532;
font-weight: bold;
}
.sy-markup.sy-deleted.sy-git_gutter {
 color: #bd2c00;
}
.sy-markup.sy-inserted.sy-git_gutter {
 color: #55a532;
}
.sy-markup.sy-changed.sy-git_gutter {
 color: #0086b3;
}
.sy-markup.sy-ignored.sy-git_gutter {
 color: #b3b3b3;
}
.sy-markup.sy-untracked.sy-git_gutter {
 color: #b3b3b3;
}
.sy-source.sy-css .sy-punctuation.sy-definition.sy-entity {
 color: #323232;
}
.sy-source.sy-css .sy-entity.sy-other.sy-attribute-name.sy-pseudo-class, .sy-source.sy-css .sy-entity.sy-other.sy-attribute-name.sy-pseudo-element {
 color: #a71d5d;
}
.sy-source.sy-css .sy-meta.sy-value, .sy-source.sy-css .sy-support.sy-constant, .sy-source.sy-css .sy-support.sy-function {
 color: #323232;
}
.sy-source.sy-css .sy-constant.sy-other.sy-color {
 color: #ed6a43;
}
.sy-source.sy-scss .sy-punctuation.sy-definition.sy-entity {
 color: #323232;
}
.sy-source.sy-scss .sy-entity.sy-other.sy-attribute-name.sy-pseudo-class, .sy-source.sy-scss .sy-entity.sy-other.sy-attribute-name.sy-pseudo-element {
 color: #a71d5d;
}
.sy-source.sy-scss .sy-support.sy-constant.sy-property-value, .sy-source.sy-scss .sy-support.sy-function {
 color: #323232;
}
.sy-source.sy-scss .sy-variable {
 color: #a71d5d;
}
.sy-variable.sy-language.sy-this.sy-js {
 color: #ed6a43;
}
.sy-source.sy-js .sy-entity.sy-name.sy-function {
 color: #323232;
}
.sy-source.sy-js .sy-meta.sy-function .sy-entity.sy-name.sy-function, .sy-source.sy-js .sy-entity.sy-name.sy-function .sy-meta.sy-function {
 color: #795da3;
font-weight: bold;
}
.sy-entity.sy-name.sy-type.sy-new.sy-js {
 color: #795da3;
}
.sy-variable.sy-language.sy-prototype.sy-js {
 color: #0086b3;
}
.sy-source.sy-js .sy-support.sy-function {
 color: #0086b3;
}
.sy-support.sy-type.sy-object.sy-console.sy-js {
 color: #795da3;
}
.sy-source.sy-python .sy-keyword {
font-weight: bold;
}
.sy-source.sy-python .sy-storage {
font-weight: bold;
}
.sy-source.sy-python .sy-storage.sy-type {
font-weight: bold;
}
.sy-source.sy-python .sy-entity.sy-name.sy-function {
 color: #323232;
font-weight: bold;
}
.sy-source.sy-php .sy-entity.sy-name.sy-type.sy-class {
 color: #323232;
font-weight: bold;
}
.sy-variable.sy-language.sy-ruby {
 color: #ed6a43;
}
.sy-entity.sy-name.sy-type.sy-module.sy-ruby {
 color: #795da3;
font-weight: bold;
}
.sy-entity.sy-name.sy-type.sy-class.sy-ruby {
 color: #795da3;
font-weight: bold;
}
.sy-entity.sy-other.sy-inherited-class.sy-ruby {
 color: #795da3;
font-weight: bold;
}
.sy-text.sy-html.sy-markdown .sy-punctuation.sy-definition {
 color: #a71d5d;
}
.sy-text.sy-html.sy-markdown .sy-meta.sy-separator {
 color: #b3b3b3;
}
.sy-text.sy-html.sy-markdown .sy-markup.sy-heading {
font-weight: bold;
}
.sy-text.sy-html.sy-markdown .sy-markup.sy-raw.sy-block {
 color: #323232;
}
.sy-text.sy-html.sy-markdown .sy-markup.sy-raw.sy-inline {
 color: #323232;
}
.sy-text.sy-html.sy-markdown .sy-meta.sy-link, .sy-text.sy-html.sy-markdown .sy-meta.sy-image {
 color: #4183c4;
}
.sy-text.sy-html.sy-markdown .sy-markup.sy-underline.sy-link, .sy-text.sy-html.sy-markdown .sy-constant.sy-other.sy-reference {
font-style: italic;
}
.sy-text.sy-html.sy-markdown .sy-markup.sy-list {
 color: #ed6a43;
}
.sy-text.sy-html.sy-markdown .sy-markup.sy-bold {
font-weight: bold;
}
.sy-text.sy-html.sy-markdown .sy-markup.sy-italic {
font-style: italic;
}
.sy-text.sy-html.sy-markdown .sy-markup.sy-bold .sy-markup.sy-italic {
font-weight: bold;
font-style: italic;
}
.sy-text.sy-html.sy-markdown .sy-markup.sy-italic .sy-markup.sy-bold {
font-weight: bold;
font-style: italic;
}

.sy-hl {
    display: inline-block;
    width: 100%;
    background-color: var(--sy-hl-bg);
}
"#;

const SYNTECT_DARK_THEME: &str = r#"
[data-theme="dark"] .sy-code {
color: #c0c5ce;
background-color: #2b303b;
}
[data-theme="dark"] .sy-variable.sy-parameter.sy-function {
color: #c0c5ce;
}
[data-theme="dark"] .sy-comment, [data-theme="dark"] .sy-punctuation.sy-definition.sy-comment {
color: #65737e;
}
[data-theme="dark"] .sy-punctuation.sy-definition.sy-string, [data-theme="dark"] .sy-punctuation.sy-definition.sy-variable, [data-theme="dark"] .sy-punctuation.sy-definition.sy-string, [data-theme="dark"] .sy-punctuation.sy-definition.sy-parameters, [data-theme="dark"] .sy-punctuation.sy-definition.sy-string, [data-theme="dark"] .sy-punctuation.sy-definition.sy-array {
color: #c0c5ce;
}
[data-theme="dark"] .sy-none {
color: #c0c5ce;
}
[data-theme="dark"] .sy-keyword.sy-operator {
color: #c0c5ce;
}
[data-theme="dark"] .sy-keyword {
color: #b48ead;
}
[data-theme="dark"] .sy-variable, [data-theme="dark"] .sy-variable.sy-other.sy-dollar.sy-only.sy-js {
color: #bf616a;
}
[data-theme="dark"] .sy-entity.sy-name.sy-function, [data-theme="dark"] .sy-meta.sy-require, [data-theme="dark"] .sy-support.sy-function.sy-any-method, [data-theme="dark"] .sy-variable.sy-function {
color: #8fa1b3;
}
[data-theme="dark"] .sy-support.sy-class, [data-theme="dark"] .sy-entity.sy-name.sy-class, [data-theme="dark"] .sy-entity.sy-name.sy-type.sy-class {
color: #ebcb8b;
}
[data-theme="dark"] .sy-meta.sy-class {
color: #eff1f5;
}
[data-theme="dark"] .sy-keyword.sy-other.sy-special-method {
color: #8fa1b3;
}
[data-theme="dark"] .sy-storage {
color: #b48ead;
}
[data-theme="dark"] .sy-support.sy-function {
color: #96b5b4;
}
[data-theme="dark"] .sy-string, [data-theme="dark"] .sy-constant.sy-other.sy-symbol, [data-theme="dark"] .sy-entity.sy-other.sy-inherited-class {
color: #a3be8c;
}
[data-theme="dark"] .sy-constant.sy-numeric {
color: #d08770;
}
[data-theme="dark"] .sy-none {
color: #d08770;
}
[data-theme="dark"] .sy-none {
color: #d08770;
}
[data-theme="dark"] .sy-constant {
color: #d08770;
}
[data-theme="dark"] .sy-entity.sy-name.sy-tag {
color: #bf616a;
}
[data-theme="dark"] .sy-entity.sy-other.sy-attribute-name {
color: #d08770;
}
[data-theme="dark"] .sy-entity.sy-other.sy-attribute-name.sy-id, [data-theme="dark"] .sy-punctuation.sy-definition.sy-entity {
color: #8fa1b3;
}
[data-theme="dark"] .sy-meta.sy-selector {
color: #b48ead;
}
[data-theme="dark"] .sy-none {
color: #d08770;
}
[data-theme="dark"] .sy-markup.sy-heading .sy-punctuation.sy-definition.sy-heading, [data-theme="dark"] .sy-entity.sy-name.sy-section {
color: #8fa1b3;
}
[data-theme="dark"] .sy-keyword.sy-other.sy-unit {
color: #d08770;
}
[data-theme="dark"] .sy-markup.sy-bold, [data-theme="dark"] .sy-punctuation.sy-definition.sy-bold {
color: #ebcb8b;
font-weight: bold;
}
[data-theme="dark"] .sy-markup.sy-italic, [data-theme="dark"] .sy-punctuation.sy-definition.sy-italic {
color: #b48ead;
font-style: italic;
}
[data-theme="dark"] .sy-markup.sy-raw.sy-inline {
color: #a3be8c;
}
[data-theme="dark"] .sy-string.sy-other.sy-link {
color: #bf616a;
}
[data-theme="dark"] .sy-meta.sy-link {
color: #d08770;
}
[data-theme="dark"] .sy-meta.sy-image {
color: #d08770;
}
[data-theme="dark"] .sy-markup.sy-list {
color: #bf616a;
}
[data-theme="dark"] .sy-markup.sy-quote {
color: #d08770;
}
[data-theme="dark"] .sy-meta.sy-separator {
color: #c0c5ce;
background-color: #4f5b66;
}
[data-theme="dark"] .sy-markup.sy-inserted, [data-theme="dark"] .sy-markup.sy-inserted.sy-git_gutter {
color: #a3be8c;
}
[data-theme="dark"] .sy-markup.sy-deleted, [data-theme="dark"] .sy-markup.sy-deleted.sy-git_gutter {
color: #bf616a;
}
[data-theme="dark"] .sy-markup.sy-changed, [data-theme="dark"] .sy-markup.sy-changed.sy-git_gutter {
color: #b48ead;
}
[data-theme="dark"] .sy-markup.sy-ignored, [data-theme="dark"] .sy-markup.sy-ignored.sy-git_gutter {
color: #4f5b66;
}
[data-theme="dark"] .sy-markup.sy-untracked, [data-theme="dark"] .sy-markup.sy-untracked.sy-git_gutter {
color: #4f5b66;
}
[data-theme="dark"] .sy-constant.sy-other.sy-color {
color: #96b5b4;
}
[data-theme="dark"] .sy-string.sy-regexp {
color: #96b5b4;
}
[data-theme="dark"] .sy-constant.sy-character.sy-escape {
color: #96b5b4;
}
[data-theme="dark"] .sy-punctuation.sy-section.sy-embedded, [data-theme="dark"] .sy-variable.sy-interpolation {
color: #ab7967;
}
[data-theme="dark"] .sy-invalid.sy-illegal {
color: #2b303b;
background-color: #bf616a;
}
[data-theme="dark"] .sy-markup.sy-deleted.sy-git_gutter {
color: #f92672;
}
[data-theme="dark"] .sy-markup.sy-inserted.sy-git_gutter {
color: #a6e22e;
}
[data-theme="dark"] .sy-markup.sy-changed.sy-git_gutter {
color: #967efb;
}
[data-theme="dark"] .sy-markup.sy-ignored.sy-git_gutter {
color: #565656;
}
[data-theme="dark"] .sy-markup.sy-untracked.sy-git_gutter {
color: #565656;
}
"#;
