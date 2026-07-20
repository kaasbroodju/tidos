use tidos::{scoped_css, view, Component, Page};

const CODE_SAMPLE: &str = r#"use tidos::{view, Component, Page};

pub struct Card {
    pub title: String,
    pub body: String,
}

impl Component for Card {
    fn to_render(&self, page: &mut Page) {
        view! {
            <div class="card">
                <h2>{&self.title}</h2>
                <p>{&self.body}</p>
            </div>
        }
    }
}"#;

const KW: &[&str] = &[
    "use", "pub", "struct", "impl", "fn", "let", "for", "mut",
    "self", "Self", "view!", "html!", "prelude",
];

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn highlight_line(line: &str) -> String {
    const C_KW: &str = "#b794f4";
    const C_STR: &str = "#9ae6b4";
    const C_TYPE: &str = "#63b3ed";
    const C_TEXT: &str = "#e2e8f0";

    let mut out = String::new();
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        let c = chars[i];
        if c == '"' {
            let mut s = String::from('"');
            i += 1;
            while i < n {
                s.push(chars[i]);
                if chars[i] == '"' { i += 1; break; }
                i += 1;
            }
            out.push_str(&format!(r#"<span style="color:{C_STR}">{}</span>"#, html_escape(&s)));
        } else if c.is_alphabetic() || c == '_' {
            let start = i;
            i += 1;
            while i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            if i < n && chars[i] == '!' {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let color = if KW.contains(&word.as_str()) {
                C_KW
            } else if word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                C_TYPE
            } else {
                C_TEXT
            };
            out.push_str(&format!(r#"<span style="color:{color}">{}</span>"#, html_escape(&word)));
        } else {
            out.push_str(&html_escape(&c.to_string()));
            i += 1;
        }
    }
    out
}

fn code_block(code: &str) -> String {
    let mut html = String::from(r#"<pre style="margin:0;font-family:'JetBrains Mono',ui-monospace,monospace;font-size:13px;line-height:1.65;color:#e2e8f0;text-wrap-mode: nowrap;white-space-collapse: preserve-spaces;">"#);
    for (i, line) in code.lines().enumerate() {
        html.push_str(&format!(
            r#"<div style="display:flex"><span style="color:rgba(240,246,251,0.25);width:28px;text-align:right;padding-right:12px;user-select:none;flex-shrink:0">{}</span><span style="flex:1">{}</span></div>"#,
            i + 1,
            highlight_line(line)
        ));
    }
    html.push_str("</pre>");
    html
}

pub struct CodeSection;

impl Component for CodeSection {
    fn to_render(&self, page: &mut Page) {
        let highlighted = code_block(CODE_SAMPLE);
        view! {
            <div class={scoped_css!("./code_section.css")}>
                <section>
                    <div class="copy">
                        <h2 class="section-title">
                            {"Components, the way "}
                            <em>{"Rust"}</em>
                            {" wants them"}
                        </h2>
                        <p class="body">
                            {"Pattern matching, iterators, conditionals, all right inside your components. No DSL to learn \u{2014} just Rust, with extra power where you want it."}
                        </p>
                        <a href="/docs/component" class="guide-link">{"See the component guide"}@html{include_str!("../hero/arrow.svg")}</a>
                    </div>
                    <div class="code-card">
                        <div class="traffic-lights">
                            <span class="dot red"></span>
                            <span class="dot yellow"></span>
                            <span class="dot green"></span>
                            <span class="filename">{"card.rs"}</span>
                        </div>
                        @html{highlighted}
                    </div>
                </section>
            </div>
        }
    }
}
