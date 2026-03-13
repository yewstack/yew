use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component]
pub fn Footer() -> Html {
    let style = css!(
        r#"
        background: var(--color-bg-secondary);
        color: var(--color-text);
        border-top: 1px solid var(--color-border);
        padding: 3rem 2rem 2rem;
        margin-top: auto;

        .inner {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 2rem;
            max-width: 1200px;
            margin: 0 auto;
        }

        .title {
            font-size: 0.875rem;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: var(--color-text);
            margin-bottom: 0.75rem;
            margin-top: 0;
        }

        .links {
            list-style: none;
            padding: 0;
            margin: 0;
        }

        .links li {
            margin-bottom: 0.5rem;
        }

        .links a {
            color: var(--color-text-secondary);
            font-size: 0.875rem;
        }

        .links a:hover {
            color: var(--color-primary);
        }

        .bottom {
            text-align: center;
            color: var(--color-text-secondary);
            font-size: 0.8125rem;
            margin-top: 2rem;
            padding-top: 1.5rem;
            border-top: 1px solid var(--color-border);
            max-width: 1200px;
            margin-left: auto;
            margin-right: auto;
        }

        .ext-icon {
            margin-left: 0.25rem;
        }
    "#
    );

    html! {
        <footer class={style}>
            <div class="inner">
                <div class="col">
                    <h4 class="title">{"Support"}</h4>
                    <ul class="links">
                        <li>
                            <a href="https://opencollective.com/yew" target="_blank" rel="noopener noreferrer">
                                {"Sponsor Project"}
                                <svg class="ext-icon" viewBox="0 0 24 24" width="13" height="13">
                                    <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                                </svg>
                            </a>
                        </li>
                    </ul>
                </div>
                <div class="col">
                    <h4 class="title">{"Participate"}</h4>
                    <ul class="links">
                        <li>
                            <a href="https://github.com/yewstack/yew" target="_blank" rel="noopener noreferrer">
                                {"GitHub"}
                                <svg class="ext-icon" viewBox="0 0 24 24" width="13" height="13">
                                    <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                                </svg>
                            </a>
                        </li>
                        <li>
                            <a href="https://discord.gg/VQck8X4" target="_blank" rel="noopener noreferrer">
                                {"Discord"}
                                <svg class="ext-icon" viewBox="0 0 24 24" width="13" height="13">
                                    <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                                </svg>
                            </a>
                        </li>
                        <li>
                            <a href="https://twitter.com/yewstack" target="_blank" rel="noopener noreferrer">
                                {"Twitter"}
                                <svg class="ext-icon" viewBox="0 0 24 24" width="13" height="13">
                                    <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                                </svg>
                            </a>
                        </li>
                    </ul>
                </div>
                <div class="col">
                    <h4 class="title">{"More"}</h4>
                    <ul class="links">
                        <li>
                            <a href="https://github.com/nickasweb/awesome-yew" target="_blank" rel="noopener noreferrer">
                                {"Yew Awesome"}
                                <svg class="ext-icon" viewBox="0 0 24 24" width="13" height="13">
                                    <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                                </svg>
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </footer>
    }
}
