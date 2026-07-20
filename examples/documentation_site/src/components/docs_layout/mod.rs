use tidos::{head, scoped_css, view, Component, Page, Slot};

pub struct DocsLayout<'a> {
    pub(crate) content: Slot<'a>
}

impl Component for DocsLayout<'_> {
    fn to_render(&self, page: &mut Page) {

        let description = "Documentation for Tidos — the SSR component framework for Rust.";
        head! {

            <meta name="description" content={description} />
            <meta property="og:type" content="website" />
            <meta property="og:site_name" content="Tidos" />
            <meta property="og:title" content="Tidos Documentation" />
            <meta property="og:description" content={description} />
            <meta property="og:image" content="https://tidos.dev/inko.png" />
            <meta property="og:image:width" content="1000" />
            <meta property="og:image:height" content="1000" />
            <meta property="og:image:alt" content="Tidos — the Inko mascot" />
            <meta name="twitter:card" content="summary" />
            <meta name="twitter:title" content="Tidos Documentation" />
            <meta name="twitter:description" content={description} />
            <meta name="twitter:image" content="https://tidos.dev/inko.png" />

            <link rel="preconnect" href="https://fonts.googleapis.com" />
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
            <link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap" rel="stylesheet" />

            <style>@html{"html,body{margin:0;padding:0;color:#fff;background-color: #0d1f2a;font-family:'Roboto'}"}</style>
            <link rel="stylesheet" href="/transitions.css" />
            <script src="/transitions.js"></script>

            <link rel="prefetch" href="/docs/getting-started" />
            <link rel="prefetch" href="/docs/the-page-macro" />
            <link rel="prefetch" href="/docs/component" />
            <link rel="prefetch" href="/docs/javascript-framework" />
            <link rel="prefetch" href="/docs/internationalization" />
            <script type="speculationrules">@html{r#"{"prefetch":[{"where":{"href_matches":"/docs/*"}}]}"#}</script>
        }

        view!{
            <div class={scoped_css!("./docs_layout.css")}>
                <div class="navbar">
                    <nav>
                        <ul>
                            <li>
                                <a href="/docs/getting-started">{"Getting started"}</a>
                            </li>
                            <li>
                                <a href="/docs/the-page-macro">{"page! macro"}</a>
                            </li>
                            <li>
                                <a href="/docs/component">{"Component"}</a>
                                <ul>
                                    <li>
                                        <a href="/docs/component#the-view-macro">{"view! macro"}</a>
                                    </li>
                                    <li>
                                        <a href="/docs/component#control-tags">{"control tags"}</a>
                                        <ul>
                                            <li>
                                                <a href="/docs/component#control-tags-conditions">{"Conditions"}</a>
                                            </li>
                                            <li>
                                                <a href="/docs/component#control-tags-loops">{"Loops"}</a>
                                            </li>
                                            <li>
                                                <a href="/docs/component#control-tags-pattern-matching">{"Pattern matching"}</a>
                                            </li>
                                            <li>
                                                <a href="/docs/component#control-tags-slots">{"Slots"}</a>
                                            </li>
                                        </ul>
                                    </li>
                                    <li>
                                        <a href="/docs/component#default-props">{"default props"}</a>
                                    </li>
                                    <li>
                                        <a href="/docs/component#the-head-macro">{"head! macro"}</a>
                                    </li>
                                    <li>
                                        <a href="/docs/component#the-scoped-css-macro">{"scoped_css! macro"}</a>
                                    </li>
                                </ul>
                            </li>
                            <li>
                                <a href="/docs/javascript-framework">{"Reactivity"}</a>
                            </li>
                            <li>
                                <a href="/docs/internationalization">{"internationalization"}</a>
                                <ul>
                                    <li>
                                        <a href="/docs/internationalization#the-i18n-macro">{"i18n!"}</a>
                                    </li>
                                </ul>
                            </li>
                        </ul>
                    </nav>
                </div>
                <div class="main-content">
                    <main>
                        @slot{self.content}
                    </main>
                </div>
            </div>
        }
    }
}