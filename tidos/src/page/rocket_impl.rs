use rocket::http::ContentType;
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
			self.head, self.template,
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
			self.lang, self.head, self.template,
		);
		Response::build_from(string.respond_to(req)?)
			.header(ContentType::HTML)
			.ok()
	}
}
