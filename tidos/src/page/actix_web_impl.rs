use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};

#[cfg(not(feature = "i18n"))]
impl Responder for crate::page::Page {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
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
		HttpResponse::Ok()
			.content_type("text/html; charset=utf-8")
			.body(string)
	}
}

#[cfg(feature = "i18n")]
impl Responder for crate::page::Page {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
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
		HttpResponse::Ok()
			.content_type("text/html; charset=utf-8")
			.body(string)
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
