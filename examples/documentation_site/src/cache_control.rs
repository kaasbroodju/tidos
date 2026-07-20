use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::ContentType;
use rocket::{Request, Response};

pub struct CacheControlFairing;

#[rocket::async_trait]
impl Fairing for CacheControlFairing {
    fn info(&self) -> Info {
        Info {
            name: "Cache-Control",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        if response.content_type() == Some(ContentType::HTML) {
            response.set_raw_header(
                "Cache-Control",
                "public, max-age=300, stale-while-revalidate=86400",
            );
        }
    }
}
