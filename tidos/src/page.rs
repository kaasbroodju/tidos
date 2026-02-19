use std::collections::HashSet;
#[cfg(all(feature = "rocket", feature = "i18n"))]
use ::rocket::request::FromParam;
#[cfg(feature = "i18n")]
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

#[cfg(not(feature = "i18n"))]
pub struct Page {
	pub head_ids: HashSet<&'static str>,
	pub head: String,
	pub template: String,
}

#[cfg(feature = "i18n")]
pub struct Page {
	pub head_ids: HashSet<&'static str>,
	pub lang: LanguageIdentifier,
	pub head: String,
	pub template: String,
}

impl Page {
	#[cfg(not(feature = "i18n"))]
	pub fn new() -> Page {
		Page {
			head_ids: HashSet::new(),
			head: String::new(),
			template: String::new(),
		}
	}

	#[cfg(feature = "i18n")]
	pub fn new(lang: LanguageIdentifier) -> Page {
		Page {
			head_ids: HashSet::new(),
			lang,
			head: String::new(),
			template: String::new(),
		}
	}

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
