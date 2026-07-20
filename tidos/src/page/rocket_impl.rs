use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::{response, Request, Response};

#[cfg(not(feature = "i18n"))]
impl<'r> Responder<'r, 'static> for crate::page::Page {
	fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
		let crate::page::Page {
			head,
			template: body,
			..
		} = self;
		let cap = 120 + head.len() + body.len();
		let mut string = String::with_capacity(cap);
		string.push_str(
			"<!doctype html>\
            <html lang=\"en\">\
                <head>\
                    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\
                    <meta charset=\"utf-8\" />",
		);
		string.push_str(&head);
		string.push_str("</head><body>");
		string.push_str(&body);
		string.push_str("</body></html>");
		Response::build_from(string.respond_to(req)?)
			.header(ContentType::HTML)
			.ok()
	}
}

#[cfg(feature = "i18n")]
impl<'r> Responder<'r, 'static> for crate::page::Page {
	fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
		let crate::page::Page {
			lang,
			head,
			template: body,
			..
		} = self;
		let lang = lang.to_string();
		let cap = 120 + lang.len() + head.len() + body.len();
		let mut string = String::with_capacity(cap);
		string.push_str("<!doctype html><html lang=\"");
		string.push_str(&lang);
		string.push_str(
			"\">\
                <head>\
                    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\
                    <meta charset=\"utf-8\" />",
		);
		string.push_str(&head);
		string.push_str("</head><body>");
		string.push_str(&body);
		string.push_str("</body></html>");
		Response::build_from(string.respond_to(req)?)
			.header(ContentType::HTML)
			.ok()
	}
}
