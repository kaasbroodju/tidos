use tidos::{scoped_css, view, Component, Page};

pub struct CodeBlock {
    pub code: String,
}

const KW: &[&str] = &[
    "use", "pub", "pub(crate)", "struct", "impl", "fn", "let", "for", "mut", "self", "Self",
    "view!", "page!", "head!", "scoped_css!", "mod", "async", "await", "return", "match", "if",
    "else", "while", "loop", "break", "continue", "in", "as", "type", "where", "const", "static",
    "enum", "trait", "crate", "super", "extern", "true", "false", "Some", "None", "Ok", "Err",
    "Box", "Vec", "String", "Option", "Result",
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
    const C_COMMENT: &str = "#718096";
    const C_NUM: &str = "#f6ad55";

    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with('#') {
        return format!(r#"<span style="color:{C_COMMENT}">{}</span>"#, html_escape(line));
    }

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
                if chars[i] == '\\' && i + 1 < n {
                    s.push(chars[i]);
                    s.push(chars[i + 1]);
                    i += 2;
                    continue;
                }
                s.push(chars[i]);
                if chars[i] == '"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push_str(&format!(r#"<span style="color:{C_STR}">{}</span>"#, html_escape(&s)));
        } else if c.is_ascii_digit() {
            let start = i;
            while i < n && (chars[i].is_ascii_alphanumeric() || chars[i] == '.') {
                i += 1;
            }
            let num: String = chars[start..i].iter().collect();
            out.push_str(&format!(r#"<span style="color:{C_NUM}">{}</span>"#, html_escape(&num)));
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
            } else if word.chars().next().map(|ch| ch.is_uppercase()).unwrap_or(false) {
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

fn render_highlighted(code: &str) -> String {
    let mut html = String::from(
        r#"<pre style="margin:0;font-family:'JetBrains Mono',ui-monospace,monospace;font-size:13px;line-height:1.65;color:#e2e8f0;text-wrap-mode: nowrap;white-space-collapse: preserve-spaces;">"#,
    );
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

impl Component for CodeBlock {
    fn to_render(&self, page: &mut Page) {
        let highlighted = render_highlighted(&self.code);
        view! {
            <div class={scoped_css!("./code_block.css")}>
                @html{highlighted}
            </div>
        }
    }
}
