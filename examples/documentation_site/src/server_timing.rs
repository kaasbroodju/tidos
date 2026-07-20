use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use std::time::Instant;

/// A Rocket fairing that injects a `Server-Timing` header on every response,
/// reporting how long Tidos took to render the page.
///
/// The timer starts when the request arrives (`on_request`) and stops the
/// moment the response is handed back (`on_response`), capturing the full
/// render cost of the `page!` macro.
///
/// The value is visible in the browser under:
///   DevTools → Network → select a request → Timing tab
pub struct ServerTimingFairing;

#[rocket::async_trait]
impl Fairing for ServerTimingFairing {
    fn info(&self) -> Info {
        Info {
            name: "Server-Timing",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        request.local_cache(|| Instant::now());
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let start = request.local_cache(|| Instant::now());
        let ms = start.elapsed().as_millis();
        response.set_raw_header(
            "Server-Timing",
            format!("render;dur={ms:.3};desc=\"Tidos\""),
        );
    }
}
