


use actix_files as fs;
use actix_web_estudos::{controllers::{camisas_controller::list, welcome_controller::welcome}, dao::db_connection::get_connection};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


extern crate diesel_migrations;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};



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

pub const MIGRATIONS : EmbeddedMigrations = embed_migrations!("./src/migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut connection =  get_connection();
    
    connection
    .run_pending_migrations(MIGRATIONS)
    .expect("Error migrating");

    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/assets", "./view/dist/assets").show_files_listing())
            .service(hello)
            .service(echo)
            .service(welcome)
            .service(actix_web::Scope::new("/camisas")
                .service(list)
            )
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
