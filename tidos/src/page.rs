use std::collections::HashSet;

pub struct Page {
	pub head_ids: HashSet<&'static str>,
	pub head: String,
	pub template: String,
}

impl Page {
	pub fn new() -> Page {
		Page {
			head_ids: HashSet::new(),
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
}
