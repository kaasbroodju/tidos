mod components;
mod pages;

#[macro_use] extern crate rocket;

use std::path::PathBuf;
use rocket::fs::{FileServer, NamedFile, relative};
use rocket::http::Header;
use rocket::Request;
use rocket::response::Responder;
use crate::pages::index::index;

#[get("/dist/<file..>")]
async fn files(file: PathBuf) -> Option<CachedFile> {
    NamedFile::open(std::path::Path::new("dist/").join(file)).await.ok().map(CachedFile)
}

struct CachedFile(NamedFile);

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for CachedFile {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut response = self.0.respond_to(req)?;
        response.set_header(Header::new("Cache-Control", "public, max-age=180"));
        Ok(response)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, files])
        .mount("/dist", FileServer::from(relative!("dist")).rank(-2))
        .mount("/public", FileServer::from(relative!("static")).rank(-3))
}