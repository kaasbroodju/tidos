use crate::components::code_block::CodeBlock;
use crate::components::nav_bar::NavBarDocs;
use crate::components::news_layout::NewsLayout;
use rocket::get;
use tidos::{head, page, view, Component, Page};

#[get("/news/v0-8-0-rc3")]
pub fn news_v0_8_0_rc3() -> Page {
	let mut page = page! {
		<NavBarDocs />
		<NewsLayout>
			{#slot:content}
				<ArticleContent />
			{/slot}
		</NewsLayout>
	};

	{
		let page = &mut page;
		head! {
			<title>{"v0.8.0-rc.3: Escaping Fixes and Clearer Errors - Tidos News"}</title>
			<meta name="description" content="Formatted text content and literal attribute values are now correctly HTML-escaped, compiler errors gained multi-suggestion output via a new TidosError type, and i18n setup mistakes now report exactly which file or value is wrong." />
		}
	}

	page
}

struct ArticleContent;

impl Component for ArticleContent {
	fn to_render(&self, page: &mut Page) {
		let code_sanitize_before = "// Before: the format! result was never passed through sanitize!\nlet name = \"<script>evil()</script>\";\nview! { <p>{\"Hi {}\", name}</p> }\n// rendered: <p>Hi <script>evil()</script></p>";
		let code_sanitize_after = "// After: the whole format! result is escaped, same as a plain {expr}\nview! { <p>{\"Hi {}\", name}</p> }\n// rendered: <p>Hi &lt;script&gt;evil()&lt;/script&gt;</p>";

		let code_class_shorthand = "view! {\n    // -> class=\"tidos-<uuid>\"\n    <div @class=\"./card.css\"></div>\n\n    // -> class=\"tidos-<uuid> extra\"\n    <div @class={\"./card.css\", \"extra\"}></div>\n}";

		let code_error_before = "thread 'main' panicked at 'Parsing failed.'";
		let code_error_after = "error: Unable to have a toggle attribute from a literal, change it into the following\n  --> src/main.rs:12:9\n   |\n12 |     :disabled=\"true\"\n   |     ^^^^^^^^^^^^^^^^\n   |\n   = help: :disabled\n   = help: :disabled={ boolean expression }\n   = help: disabled=\"true\"\n   = help: disabled={ \"true\" }";

		let code_i18n_error = "Tidos i18n: could not read resource directory '/srv/app/locales': No such file or directory (os error 2)\nhint: `resource_location` in Tidos.toml is resolved relative to the process's current working directory.";

		let code_cargo = "[dependencies]\ntidos = \"0.8.0-rc.3\"";

		view! {
			<div class="article-content">
				<a class="back-link" href="/news">{"← All news"}</a>

				<h1>{"Escaping Fixes and Clearer Errors"}</h1>
				<div class="article-meta">
					<span class="article-version">{"v0.8.0-rc.3"}</span>
					<span>{"August 19, 2026"}</span>
					<span>{"Third release candidate"}</span>
				</div>

				<p>{"v0.8.0-rc.3 closes an escaping gap left over from rc.2's rewrite of attribute and text-content code generation, and spends the rest of the cycle on diagnostics: a new error type with multi-line suggestions, and i18n setup failures that now name the exact file or value at fault instead of just panicking."}</p>

				<h2>{"Fix: formatted text and literal attributes weren't escaped"}<span class="fix-badge">{"Fix"}</span></h2>
				<p>{"A formatted text expression, {\"...{}\", args}, was building its output with format! but never routing the result through sanitize!, so interpolated values were written into the page unescaped. Constant literal attribute values (class=\"...\") had the same gap. Both now go through the same escaping path as every other text and attribute form:"}</p>
				<CodeBlock code={code_sanitize_before.to_string()} />
				<CodeBlock code={code_sanitize_after.to_string()} />
				<p>{"If you were relying on the old, unescaped output anywhere, switch to @html{...} at that call site to keep it explicit. Everywhere else this is a silent correctness fix, no code changes needed."}</p>

				<h2>{"New: @class shorthand"}</h2>
				<p>{"@ prefixed on class (and only class) is sugar for scoped_css!. Because the generated class name is a compile-time literal instead of an opaque expression, it merges into the surrounding static markup instead of forcing a dynamic push:"}</p>
				<CodeBlock code={code_class_shorthand.to_string()} />
				<p>{"Anything after the path is parsed with the same literal / format-string / expression grammar as a plain class={...} value, and appended space-separated."}</p>

				<h2>{"Clearer compiler errors"}<span class="qol-badge">{"Quality of life"}</span></h2>
				<p>{"Parsing errors used to bottom out in a handful of internal .unwrap() calls, or a single format!'d string when a diagnostic needed to suggest more than one fix. Both are gone: parsing no longer panics on malformed input, and a new TidosError type carries a list of suggestions that rustc renders as separate help lines:"}</p>
				<CodeBlock code={code_error_before.to_string()} />
				<CodeBlock code={code_error_after.to_string()} />

				<h2>{"i18n errors now name the file"}<span class="fix-badge">{"Fix"}</span></h2>
				<p>{"A broken Tidos.toml, an invalid default_locale, or a locale directory that isn't a valid language tag used to surface as a bare 'Parsing failed' or 'Unable to open resources' panic at startup. TidosI18nError now carries the resolved path or value that was actually the problem:"}</p>
				<CodeBlock code={code_i18n_error.to_string()} />

				<h2>{"Under the hood"}</h2>
				<p>{"Attribute code generation now flows through the same flat-args accumulator as text content, concatenating adjacent static pieces before the combine! call instead of after. Deeply nested view! trees no longer risk hitting rustc's macro recursion limit. The IsStatic trait, made redundant by that change, was removed."}</p>

				<h2>{"Upgrading"}</h2>
				<p>{"Update your Cargo.toml to the third release candidate:"}</p>
				<CodeBlock code={code_cargo.to_string()} />
				<p>{"No API changes this cycle. Double-check any formatted text content or literal attribute values that contain &, <, >, \" or ' characters, since those now render escaped where they previously didn't."}</p>
			</div>
		}
	}
}
