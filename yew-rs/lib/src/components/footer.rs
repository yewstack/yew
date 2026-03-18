use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component]
pub fn Footer() -> Html {
    html! {
        <footer class={css!("
            background: var(--color-bg-secondary);
            color: var(--color-text);
            border-top: 1px solid var(--color-border);
            padding: 3rem 2rem 2rem;
            margin-top: auto;
        ")}>
            <div class={css!("
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                gap: 2rem;
                max-width: 1200px;
                margin: 0 auto;
            ")}>
                <div>
                    <h4 class={css!(r#"
                        font-size: 0.875rem;
                        text-transform: uppercase;
                        letter-spacing: 0.05em;
                        color: var(--color-text);
                        margin-bottom: 0.75rem;
                        margin-top: 0;
                    "#)}>{"Support"}</h4>
                    <ul class={css!("list-style: none; padding: 0; margin: 0;")}>
                        <li class={css!("margin-bottom: 0.5rem;")}>
                            <a href="https://opencollective.com/yew" target="_blank" rel="noopener noreferrer"
                                class={css!("color: var(--color-text-secondary); font-size: 0.875rem; &:hover { color: var(--color-primary); }")}>
                                {"Sponsor Project"}
                                <svg class={css!("margin-left: 0.25rem;")} viewBox="0 0 24 24" width="13" height="13">
                                    <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                                </svg>
                            </a>
                        </li>
                    </ul>
                </div>
                <div>
                    <h4 class={css!(r#"
                        font-size: 0.875rem;
                        text-transform: uppercase;
                        letter-spacing: 0.05em;
                        color: var(--color-text);
                        margin-bottom: 0.75rem;
                        margin-top: 0;
                    "#)}>{"Participate"}</h4>
                    <ul class={css!("list-style: none; padding: 0; margin: 0;")}>
                        <li class={css!("margin-bottom: 0.5rem;")}>
                            <a href="https://github.com/yewstack/yew" target="_blank" rel="noopener noreferrer"
                                class={css!("color: var(--color-text-secondary); font-size: 0.875rem; &:hover { color: var(--color-primary); }")}>
                                {"GitHub"}
                                <svg class={css!("margin-left: 0.25rem;")} viewBox="0 0 24 24" width="13" height="13">
                                    <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                                </svg>
                            </a>
                        </li>
                        <li class={css!("margin-bottom: 0.5rem;")}>
                            <a href="https://discord.gg/VQck8X4" target="_blank" rel="noopener noreferrer"
                                class={css!("color: var(--color-text-secondary); font-size: 0.875rem; &:hover { color: var(--color-primary); }")}>
                                {"Discord"}
                                <svg class={css!("margin-left: 0.25rem;")} viewBox="0 0 24 24" width="13" height="13">
                                    <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                                </svg>
                            </a>
                        </li>
                        <li class={css!("margin-bottom: 0.5rem;")}>
                            <a href="https://twitter.com/yewstack" target="_blank" rel="noopener noreferrer"
                                class={css!("color: var(--color-text-secondary); font-size: 0.875rem; &:hover { color: var(--color-primary); }")}>
                                {"Twitter"}
                                <svg class={css!("margin-left: 0.25rem;")} viewBox="0 0 24 24" width="13" height="13">
                                    <path fill="currentColor" d="M21 13v10h-21v-19h12v2h-10v15h17v-8h2zm3-12h-10.988l4.035 4-6.977 7.07 2.828 2.828 6.977-7.07 4.125 4.172v-11z"/>
                                </svg>
                            </a>
                        </li>
                    </ul>
                </div>
                <div>
                    <h4 class={css!(r#"
                        font-size: 0.875rem;
                        text-transform: uppercase;
                        letter-spacing: 0.05em;
                        color: var(--color-text);
                        margin-bottom: 0.75rem;
                        margin-top: 0;
                    "#)}>{"More"}</h4>
                    <ul class={css!("list-style: none; padding: 0; margin: 0;")}>
                        <li class={css!("margin-bottom: 0.5rem;")}>
                            <a href="https://github.com/nickasweb/awesome-yew" target="_blank" rel="noopener noreferrer"
                                class={css!("color: var(--color-text-secondary); font-size: 0.875rem; &:hover { color: var(--color-primary); }")}>
                                {"Yew Awesome"}
                                <svg class={css!("margin-left: 0.25rem;")} viewBox="0 0 24 24" width="13" height="13">
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
