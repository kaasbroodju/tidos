use warp::reply::Reply;

#[cfg(not(feature = "i18n"))]
impl Reply for crate::page::Page {
	fn into_response(self) -> warp::reply::Response {
		warp::reply::html(format!(
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
impl Reply for crate::page::Page {
	fn into_response(self) -> warp::reply::Response {
		warp::reply::html(format!(
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
