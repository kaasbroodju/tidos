use std::borrow::Cow;
use std::collections::HashSet;
#[cfg(feature = "i18n")]
use unic_langid::LanguageIdentifier;
#[cfg(feature = "i18n")]
use unic_langid::LanguageIdentifierError;

#[cfg(feature = "rocket")]
mod rocket_impl;

#[cfg(feature = "axum")]
mod axum_impl;

#[cfg(feature = "actix-web")]
mod actix_web_impl;

#[cfg(feature = "warp")]
mod warp_impl;

/// A fully rendered page ready to be returned from a route handler.
///
/// `Page` renders HTML by appending every fragment directly into a single
/// growing `String` buffer.  Static and dynamic content both go through
/// [`push_static`](Page::push_static), so there is at most one allocation for
/// the buffer itself (plus amortised reallocations as it grows) and zero
/// per-fragment allocations.  The final HTML is obtained by calling
/// [`into_html`](Page::into_html), which moves the buffer out without copying.
#[cfg(not(feature = "i18n"))]
pub struct Page {
	/// Tracks which `<head>` element IDs have already been injected, to avoid
	/// duplicates (e.g. when a component is rendered in a loop).
	pub head_ids: HashSet<&'static str>,
	/// Accumulated HTML content for the `<head>` element.
	pub head: String,
	/// The HTML output buffer.  All push operations append directly here.
	pub template: String,
}

/// A fully rendered page ready to be returned from a route handler.
///
/// `Page` renders HTML by appending every fragment directly into a single
/// growing `String` buffer.  Static and dynamic content both go through
/// [`push_static`](Page::push_static), so there is at most one allocation for
/// the buffer itself (plus amortised reallocations as it grows) and zero
/// per-fragment allocations.  The final HTML is obtained by calling
/// [`into_html`](Page::into_html), which moves the buffer out without copying.
#[cfg(feature = "i18n")]
pub struct Page {
	/// Tracks which `<head>` element IDs have already been injected, to avoid
	/// duplicates (e.g. when a component is rendered in a loop).
	pub head_ids: HashSet<&'static str>,
	/// The negotiated locale for this page, used in the `<html lang="…">` attribute.
	pub lang: LanguageIdentifier,
	/// Accumulated HTML content for the `<head>` element.
	pub head: String,
	/// The HTML output buffer.  All push operations append directly here.
	pub template: String,
}

/// Initial capacity of the HTML output buffer, chosen to fit in one OS memory page.
const PAGE_SIZE: usize = 4096;

impl Page {
	#[cfg(not(feature = "i18n"))]
	#[allow(clippy::new_without_default)]
	pub fn new() -> Page {
		Page {
			head_ids: HashSet::new(),
			head: String::new(),
			template: String::with_capacity(PAGE_SIZE),
		}
	}

	#[cfg(feature = "i18n")]
	pub fn new(lang: LanguageIdentifier) -> Page {
		Page {
			head_ids: HashSet::new(),
			lang,
			head: String::new(),
			template: String::with_capacity(PAGE_SIZE),
		}
	}

	/// Append a string slice to the template buffer.
	///
	/// Both static (`&'static str`) and dynamic (`&str`, `String`) content
	/// flow through this method — with the `String` buffer design there is no
	/// reason to distinguish the two at runtime.
	#[inline]
	pub fn push_static(&mut self, s: &str) {
		self.template.push_str(s);
	}

	/// Append an owned `String` to the template buffer.
	///
	/// The string is consumed and its bytes are copied into the buffer.
	/// Prefer [`push_static`](Page::push_static) when you already have a `&str`.
	#[inline]
	pub fn push_dynamic(&mut self, s: String) {
		self.template.push_str(&s);
	}

	/// Injects `element` into the page `<head>`, keyed by `id`.
	///
	/// If `id` has already been inserted the element is silently ignored,
	/// which makes it safe to call this method from components rendered inside
	/// a loop (e.g. via [`scoped_css!`](macro@crate::scoped_css)).
	pub fn add_elements_to_head(&mut self, id: &'static str, element: String) {
		if self.head_ids.insert(id) {
			self.head += &element;
		}
	}

	/// Consume the page and return the rendered HTML.
	///
	/// This simply moves the internal buffer — no additional allocation or
	/// copying occurs.
	pub fn into_html(self) -> String {
		self.template
	}
}

/// Dispatch trait used by the [`combine!`](macro@crate::combine) macro to push a
/// value into a [`Page`] without requiring the call site to know the concrete type.
///
/// - `&str` / `&'static str` → [`Page::push_static`]
/// - `String` → [`Page::push_dynamic`]
/// - `Cow<'_, str>` → zero allocation when the cow is `Borrowed`
pub trait PushIntoPage {
	fn push_into_page(self, page: &mut Page);
}

impl PushIntoPage for &str {
	#[inline]
	fn push_into_page(self, page: &mut Page) {
		if !self.is_empty() {
			page.push_static(self);
		}
	}
}

impl PushIntoPage for String {
	#[inline]
	fn push_into_page(self, page: &mut Page) {
		if !self.is_empty() {
			page.push_dynamic(self);
		}
	}
}

impl<'a> PushIntoPage for Cow<'a, str> {
	#[inline]
	fn push_into_page(self, page: &mut Page) {
		let s = self.as_ref();
		if !s.is_empty() {
			page.push_static(s);
		}
	}
}

/// A request guard / extractor that parses a locale from a URL path segment.
///
/// Available whenever the `i18n` feature is enabled. Framework-specific
/// extractor implementations are provided by the `rocket`, `axum`, and
/// `actix-web` features. For `warp`, `Lang` implements [`std::str::FromStr`]
/// so it works directly with `warp::path::param::<Lang>()` and
/// `warp::header::<Lang>("accept-language")`.
///
/// The locale string (e.g. `"en-US"`, `"nl-NL"`) is parsed into a
/// [`LanguageIdentifier`] and made available on the [`Page`] so that
/// [`i18n!`](crate::i18n::i18n) can resolve translations.
///
/// **Rocket** — use as a `FromParam` route parameter: `#[get("/<lang>")]`.
///
/// **Axum** — use as a `FromRequestParts` extractor; route must have a
/// `:lang` path segment (e.g. `"/:lang"`).
///
/// **Actix Web** — use as a `FromRequest` extractor; route must have a
/// `{lang}` path segment (e.g. `"/{lang}"`).
///
/// **Warp** — use `warp::path::param::<Lang>()` or
/// `warp::header::<Lang>("accept-language")`.
#[cfg(feature = "i18n")]
pub struct Lang(pub LanguageIdentifier);

#[cfg(feature = "i18n")]
impl std::str::FromStr for Lang {
	type Err = LanguageIdentifierError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.parse::<LanguageIdentifier>().map(Lang)
	}
}

#[cfg(all(feature = "rocket", feature = "i18n"))]
impl<'a> ::rocket::request::FromParam<'a> for Lang {
	type Error = LanguageIdentifierError;

	fn from_param(param: &'a str) -> Result<Self, Self::Error> {
		match LanguageIdentifier::from_bytes(param.as_bytes()) {
			Ok(id) => Ok(Self(id)),
			Err(error) => Err(error),
		}
	}
}
