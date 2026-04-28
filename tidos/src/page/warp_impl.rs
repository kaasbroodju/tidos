use warp::reply::Reply;

#[cfg(not(feature = "i18n"))]
impl Reply for crate::page::Page {
	fn into_response(self) -> warp::reply::Response {
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
		warp::reply::html(string).into_response()
	}
}

#[cfg(feature = "i18n")]
impl Reply for crate::page::Page {
	fn into_response(self) -> warp::reply::Response {
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
		warp::reply::html(string).into_response()
	}
}
