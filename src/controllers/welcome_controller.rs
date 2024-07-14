use actix_files as fs;
use std::path::PathBuf;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
pub async fn welcome(_req: HttpRequest) ->  Result<fs::NamedFile, std::io::Error> {
    let path: PathBuf = "./view/dist/index.html".parse().unwrap();
    fs::NamedFile::open(path)
}