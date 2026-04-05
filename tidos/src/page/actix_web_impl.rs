use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};

#[cfg(not(feature = "i18n"))]
impl Responder for crate::page::Page {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		HttpResponse::Ok()
			.content_type("text/html; charset=utf-8")
			.body(format!(
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
			))
	}
}

#[cfg(feature = "i18n")]
impl Responder for crate::page::Page {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		HttpResponse::Ok()
			.content_type("text/html; charset=utf-8")
			.body(format!(
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
			))
	}
}

#[cfg(feature = "i18n")]
impl actix_web::FromRequest for crate::page::Lang {
	type Error = actix_web::Error;
	type Future = std::future::Ready<Result<Self, Self::Error>>;

	fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
		use unic_langid::LanguageIdentifier;

		let result = req
			.match_info()
			.get("lang")
			.ok_or_else(|| actix_web::error::ErrorBadRequest("missing lang"))
			.and_then(|s| {
				LanguageIdentifier::from_bytes(s.as_bytes())
					.map(crate::page::Lang)
					.map_err(|_| actix_web::error::ErrorBadRequest("invalid lang"))
			});

		std::future::ready(result)
	}
}
