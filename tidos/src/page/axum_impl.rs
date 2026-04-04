use axum::response::{Html, IntoResponse, Response};

#[cfg(not(feature = "i18n"))]
impl IntoResponse for crate::page::Page {
	fn into_response(self) -> Response {
		Html(format!(
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
		.into_response()
	}
}

#[cfg(feature = "i18n")]
impl IntoResponse for crate::page::Page {
	fn into_response(self) -> Response {
		Html(format!(
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
		.into_response()
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
