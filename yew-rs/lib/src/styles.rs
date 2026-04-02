use yew::prelude::*;
use yew_site_proc::comp;

#[comp]
pub fn GlobalStyles() {
    html! {
        <stylist::yew::Global css={css!(r#"
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

            "#)} />
    }
}
