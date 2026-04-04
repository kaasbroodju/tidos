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
/// `Page` accumulates the rendered HTML body and any `<head>` elements (such
/// as `<style>` blocks from [`scoped_css!`](macro@crate::scoped_css) or custom
/// tags from [`head!`](macro@crate::head)). When the `rocket` feature is
/// enabled, `Page` implements Rocket's `Responder` and produces a complete
/// `<!doctype html>` document.
///
/// You rarely construct `Page` directly — the
/// [`page!`](macro@crate::page) macro creates one for you and returns it
/// from the route handler.
#[cfg(not(feature = "i18n"))]
pub struct Page {
	/// Tracks which `<head>` element IDs have already been injected, to avoid
	/// duplicates (e.g. when a component is rendered in a loop).
	pub head_ids: HashSet<&'static str>,
	/// Accumulated HTML content for the `<head>` element.
	pub head: String,
	/// Accumulated HTML content for the `<body>` element.
	pub template: String,
}

/// A fully rendered page ready to be returned from a route handler.
///
/// `Page` accumulates the rendered HTML body and any `<head>` elements (such
/// as `<style>` blocks from [`scoped_css!`](macro@crate::scoped_css) or custom
/// tags from [`head!`](macro@crate::head)). When a web framework feature is
/// enabled, `Page` implements that framework's response trait and produces a
/// complete `<!doctype html>` document with the correct `lang` attribute set
/// from the negotiated locale.
///
/// You rarely construct `Page` directly — the
/// [`page!`](macro@crate::page) macro creates one for you and returns it
/// from the route handler.
#[cfg(feature = "i18n")]
pub struct Page {
	/// Tracks which `<head>` element IDs have already been injected, to avoid
	/// duplicates (e.g. when a component is rendered in a loop).
	pub head_ids: HashSet<&'static str>,
	/// The negotiated locale for this page, used in the `<html lang="…">` attribute.
	pub lang: LanguageIdentifier,
	/// Accumulated HTML content for the `<head>` element.
	pub head: String,
	/// Accumulated HTML content for the `<body>` element.
	pub template: String,
}

impl Page {
	/// Creates a new, empty [`Page`].
	///
	/// Called internally by the [`page!`](macro@crate::page) macro.
	#[cfg(not(feature = "i18n"))]
	#[allow(clippy::new_without_default)]
	pub fn new() -> Page {
		Page {
			head_ids: HashSet::new(),
			head: String::new(),
			template: String::new(),
		}
	}

	/// Creates a new, empty [`Page`] for the given locale.
	///
	/// Called internally by the [`page!`](macro@crate::page) macro when the
	/// `i18n` feature is enabled. The `lang` value is extracted from the route
	/// parameter by [`Lang`].
	#[cfg(feature = "i18n")]
	pub fn new(lang: LanguageIdentifier) -> Page {
		Page {
			head_ids: HashSet::new(),
			lang,
			head: String::new(),
			template: String::new(),
		}
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
