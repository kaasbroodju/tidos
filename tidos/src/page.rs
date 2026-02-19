use std::collections::HashSet;
#[cfg(all(feature = "rocket", feature = "i18n"))]
use ::rocket::request::FromParam;
#[cfg(feature = "i18n")]
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

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
/// tags from [`head!`](macro@crate::head)). When the `rocket` feature is
/// enabled, `Page` implements Rocket's `Responder` and produces a complete
/// `<!doctype html>` document with the correct `lang` attribute set from the
/// negotiated locale.
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

#[cfg(feature = "rocket")]
mod rocket {
	use rocket::http::{ContentType, Status};
	use rocket::response::Responder;
	use rocket::{response, Request, Response};

	#[cfg(not(feature = "i18n"))]
	impl<'r> Responder<'r, 'static> for crate::page::Page {
		fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
			let string = format!(
				"<!doctype html>\
                <html lang=\"en\">\
                    <head>\
                        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\
                        <meta charset=\"utf-8\" />\
                        {}\
                    </head>\
                    <body>\
                        {}\
                    </body>\
                </html>",
				self.head,
				self.template,
			);
			Response::build_from(string.respond_to(req)?)
				.header(ContentType::HTML)
				.ok()
		}
	}

	#[cfg(feature = "i18n")]
	impl<'r> Responder<'r, 'static> for crate::page::Page {
		fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
			let string = format!(
				"<!doctype html>\
                <html lang=\"{}\">\
                    <head>\
                        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\
                        <meta charset=\"utf-8\" />\
                        {}\
                    </head>\
                    <body>\
                        {}\
                    </body>\
                </html>",
				self.lang,
				self.head,
				self.template,
			);
			Response::build_from(string.respond_to(req)?)
				.header(ContentType::HTML)
				.ok()
		}
	}
}

/// A Rocket request guard that extracts a locale from the first URL path segment.
///
/// Use `lang: Lang` as a route parameter when the `rocket` and `i18n` features
/// are both enabled. The locale string (e.g. `"en-US"`, `"nl-NL"`) is parsed
/// into a [`LanguageIdentifier`] and made available on the [`Page`] so that
/// [`i18n!`](crate::i18n::i18n) can resolve translations.
///
/// # Example
///
/// ```rust,no_run
/// use tidos::i18n::Lang;
/// use tidos::{page, Page};
///
/// #[get("/<lang>")]
/// pub fn index(lang: Lang) -> Page {
///     page! {
///         <main><h1>Hello</h1></main>
///     }
/// }
/// ```
#[cfg(all(feature = "rocket", feature = "i18n"))]
pub struct Lang(pub LanguageIdentifier);

#[cfg(all(feature = "rocket", feature = "i18n"))]
impl<'a> FromParam<'a> for Lang {
	type Error = LanguageIdentifierError;

	fn from_param(param: &'a str) -> Result<Self, Self::Error> {
		match LanguageIdentifier::from_bytes(param.as_bytes()) {
			Ok(id) => Ok(Self(id)),
			Err(error) => Err(error)
		}
	}
}