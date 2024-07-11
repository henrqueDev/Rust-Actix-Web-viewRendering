use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn welcome(_req: HttpRequest) ->  Result<NamedFile, std::io::Error> {
    let path: PathBuf = "./index.html".parse().unwrap();
    NamedFile::open(path)
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/hellou")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Its me...")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(welcome)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8999))?
    .run()
    .await
}
