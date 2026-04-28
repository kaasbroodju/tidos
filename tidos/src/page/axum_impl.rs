use axum::response::{Html, IntoResponse, Response};

#[cfg(not(feature = "i18n"))]
impl IntoResponse for crate::page::Page {
	fn into_response(self) -> Response {
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
		Html(string).into_response()
	}
}

#[cfg(feature = "i18n")]
impl IntoResponse for crate::page::Page {
	fn into_response(self) -> Response {
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
		Html(string).into_response()
	}
}

#[cfg(feature = "i18n")]
impl<S> axum::extract::FromRequestParts<S> for crate::page::Lang
where
	S: Send + Sync,
{
	type Rejection = Response;

	async fn from_request_parts(
		parts: &mut axum::http::request::Parts,
		state: &S,
	) -> Result<Self, Self::Rejection> {
		use axum::extract::Path;
		use axum::response::IntoResponse as _;
		use std::collections::HashMap;
		use unic_langid::LanguageIdentifier;

		let Path(params) = Path::<HashMap<String, String>>::from_request_parts(parts, state)
			.await
			.map_err(|e| e.into_response())?;

		let lang_str = params
			.get("lang")
			.ok_or_else(|| axum::http::StatusCode::BAD_REQUEST.into_response())?;

		LanguageIdentifier::from_bytes(lang_str.as_bytes())
			.map(crate::page::Lang)
			.map_err(|_| axum::http::StatusCode::BAD_REQUEST.into_response())
	}
}
