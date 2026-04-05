#![doc = include_str!("lib.md")]

mod components;
mod page;

#[cfg(feature = "i18n")]
mod i18n_config;
#[doc(hidden)]
pub mod internals;

pub use components::Component;
#[doc(hidden)]
pub use internals::sanitize;
pub use page::Page;

#[doc(inline)]
pub use tidos_macro::*;

#[doc(hidden)]
#[macro_export]
macro_rules! sanitize {
	($input:expr) => {
		tidos::internals::sanitize(&$input)
	};
}

#[macro_export]
macro_rules! combine {
	// Arguments can be any of the following:
	// * a string literal          — e.g. "<div>", "hello"
	// * a sanitize! call          — returns &str
	// * a &String / &str ref      — e.g. &component.to_render(page)
	// * any expression returning String or &str

	// -----------------------------------------------------------------------
	// Literal-only shortcut
	//
	// When every argument is a string literal, `concat!` can merge them all
	// at compile time into a single &'static str — no String, no allocation.
	// This rule must come before the general entry point because a literal
	// is also a valid `expr`, and the more specific rule wins when listed first.
	//
	// combine!("Hello ", "World!") => concat!("Hello ", "World!") => "Hello World!"
	// -----------------------------------------------------------------------
	($a:literal, $b:literal) => { concat!($a, $b) };

	// -----------------------------------------------------------------------
	// Entry point
	//
	// The first argument is always a String that acts as the accumulator.
	// Every subsequent argument is appended to it via the `+` operator.
	// Rust only allows `String + &str`, so the left-hand side must always
	// be a String — that is why the first argument is the accumulator.
	//
	// combine!(String::new(), "a", expr, "b", "c")
	//   => @acc String::new(); "a", expr, "b", "c"
	// -----------------------------------------------------------------------
	($acc:expr, $($rest:expr),+) => {
		$crate::combine!(@acc $acc; $($rest),+)
	};
	// Base cases when combine! is called with a single argument.
	($single:literal) => { $single };
	($single:expr)    => { $single };

	// -----------------------------------------------------------------------
	// Phase 1 — @acc (accumulator)
	//
	// Walks left-to-right through the remaining arguments, appending each
	// one to the accumulator with `+`.  Adjacent string literals are handed
	// off to @concat so they can be merged into a single `concat!` call at
	// compile time instead of allocating an intermediate String.
	//
	// The accumulator is always a String (from the entry point or from a
	// previous `String + &str` step), so `acc + literal` and `acc + &expr`
	// are always valid.
	// -----------------------------------------------------------------------

	// Two adjacent literals with more after them:
	//   @acc acc; "a", "b", rest.. => @concat ["a","b"] acc; rest..
	(@acc $acc:expr; $a:literal, $b:literal, $($rest:expr),+) => {
		$crate::combine!(@concat [$a, $b] $acc; $($rest),+)
	};
	// Exactly two adjacent literals, nothing after:
	//   @acc acc; "a", "b" => acc + concat!("a", "b")
	(@acc $acc:expr; $a:literal, $b:literal) => {
		$acc + concat!($a, $b)
	};
	// Single literal followed by more:
	//   @acc acc; "a", rest.. => @acc acc + "a"; rest..
	(@acc $acc:expr; $lit:literal, $($rest:expr),+) => {
		$crate::combine!(@acc $acc + $lit; $($rest),+)
	};
	// &expr followed by more (already a reference — do not add another &):
	//   @acc acc; &e, rest.. => @acc acc + &e; rest..
	(@acc $acc:expr; &$e:expr, $($rest:expr),+) => {
		$crate::combine!(@acc $acc + &$e; $($rest),+)
	};
	// Any other expression followed by more (add & to borrow it as &str):
	//   @acc acc; expr, rest.. => @acc acc + &expr; rest..
	(@acc $acc:expr; $e:expr, $($rest:expr),+) => {
		$crate::combine!(@acc $acc + &$e; $($rest),+)
	};
	// Base cases — nothing left after this argument:
	(@acc $acc:expr; $lit:literal) => { $acc + $lit };
	(@acc $acc:expr; &$e:expr)     => { $acc + &$e  };
	(@acc $acc:expr; $e:expr)      => { $acc + &$e  };

	// -----------------------------------------------------------------------
	// Phase 2 — @concat (literal collector)
	//
	// Entered when two or more adjacent literals are seen.  Keeps collecting
	// further adjacent literals into the bracket list so they can all be
	// passed to a single `concat!` at the end — a compile-time operation
	// that produces a &'static str with zero runtime allocation.
	//
	// As soon as a non-literal is encountered the collected literals are
	// flushed as concat![..] and control returns to @acc.
	// -----------------------------------------------------------------------

	// Next item is also a literal — keep collecting:
	//   @concat ["a","b"] acc; "c", rest.. => @concat ["a","b","c"] acc; rest..
	(@concat [$($lits:literal),+] $acc:expr; $next:literal, $($rest:expr),+) => {
		$crate::combine!(@concat [$($lits),+, $next] $acc; $($rest),+)
	};
	// Last item is a literal — flush everything into one concat!:
	//   @concat ["a","b"] acc; "c" => acc + concat!("a","b","c")
	(@concat [$($lits:literal),+] $acc:expr; $next:literal) => {
		$acc + concat!($($lits),+, $next)
	};
	// Non-literal &expr with more after — flush and return to @acc:
	//   @concat ["a","b"] acc; &e, rest.. => @acc acc + concat!("a","b"); &e, rest..
	(@concat [$($lits:literal),+] $acc:expr; &$e:expr, $($rest:expr),+) => {
		$crate::combine!(@acc $acc + concat!($($lits),+); &$e, $($rest),+)
	};
	// Non-literal &expr, nothing after — flush and append:
	//   @concat ["a","b"] acc; &e => acc + concat!("a","b") + &e
	(@concat [$($lits:literal),+] $acc:expr; &$e:expr) => {
		$acc + concat!($($lits),+) + &$e
	};
	// Non-literal expr with more after — flush and return to @acc:
	//   @concat ["a","b"] acc; expr, rest.. => @acc acc + concat!("a","b"); expr, rest..
	(@concat [$($lits:literal),+] $acc:expr; $e:expr, $($rest:expr),+) => {
		$crate::combine!(@acc $acc + concat!($($lits),+); $e, $($rest),+)
	};
	// Non-literal expr, nothing after — flush and append:
	//   @concat ["a","b"] acc; expr => acc + concat!("a","b") + &expr
	(@concat [$($lits:literal),+] $acc:expr; $e:expr) => {
		$acc + concat!($($lits),+) + &$e
	};
	// Nothing left at all — just flush:
	//   @concat ["a","b"] acc; => acc + concat!("a","b")
	(@concat [$($lits:literal),+] $acc:expr;) => {
		$acc + concat!($($lits),+)
	};
}

/// Internationalization support for Tidos, backed by [Fluent](https://projectfluent.org/).
///
/// Enable with `features = ["i18n"]` in `Cargo.toml`.
///
/// # Setup
///
/// **1.** Call [`enable_i18n!`] once at the top level of `main.rs`.
///
/// **2.** Create a `Tidos.toml` in the project root:
///
/// ```toml
/// [default]
/// resource_location = "translations"
/// default_locale    = "en-US"
/// resources         = ["common.ftl"]
/// ```
///
/// **3.** Place `.ftl` files under `resource_location`, grouped by locale:
///
/// ```text
/// translations/
/// ├── en-US/
/// │   └── common.ftl
/// └── nl-NL/
///     └── common.ftl
/// ```
///
/// **4.** Add `lang: Lang` to route handlers (locale is the first path
/// segment, e.g. `/en-US`), then call [`i18n!`] inside any component.
///
/// # Example
///
/// ```rust,no_run
/// use tidos::i18n::{enable_i18n, i18n, Lang};
/// use tidos::{view, page, Component, Page};
///
/// enable_i18n!();
///
/// pub struct Greeting;
///
/// impl Component for Greeting {
///     fn to_render(&self, page: &mut Page) -> String {
///         view! {
///             <h1>{i18n!("greeting")}</h1>
///             <p>{i18n!("shared-photos", ("userName", "Anne"), ("photoCount", 3))}</p>
///         }
///     }
/// }
///
/// #[get("/<lang>")]
/// pub fn index(lang: Lang) -> Page {
///     page! { <Greeting /> }
/// }
/// ```
#[cfg(feature = "i18n")]
pub mod i18n {
	pub use fluent;
	pub use fluent_langneg;
	pub use fluent_resmgr;
	pub use lazy_static;
	pub use unic_langid;

	/// Initialises the Tidos i18n translation system.
	///
	/// Call this macro **once**, at the top level of `main.rs`, before
	/// launching your web server. It creates a lazily-initialised global that
	/// reads [`TidosI18nConfig`] from `Tidos.toml` (or environment variables
	/// prefixed with `TIDOS_I18N_`).
	///
	/// # Example
	///
	/// ```rust,no_run
	/// use tidos::i18n::enable_i18n;
	///
	/// enable_i18n!();
	///
	/// #[rocket::main]
	/// async fn main() {
	///     // launch Rocket here
	/// }
	/// ```
	#[macro_export]
	macro_rules! enable_i18n {
		() => {
			tidos::i18n::lazy_static::lazy_static! {
				static ref TIDOS_I18N_CONFIGURATION: tidos::i18n::TidosI18nConfig = {
					tidos::i18n::TidosI18nConfig::figment()
						.extract()
						.unwrap()
				};
			}
		};
	}
	pub use crate::enable_i18n;

	#[doc(inline)]
	pub use tidos_i18n::*;

	pub use crate::i18n_config::TidosI18nConfig;

	pub use crate::page::Lang;
}
