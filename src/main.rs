use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use meilisearch_sdk::{client::*, document::*, search::*};

mod db;

#[get("/api/{word}")]
async fn hello(web::Path(word): web::Path<String>) -> impl Responder {
    let response: Vec<SearchResult<db::Word>> = db::search(word);
    HttpResponse::Ok().body(format!("Hello {:#?}!", response))
}

// #[post("/load")]
// async fn echo() {
//     db::load()
// }

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            // .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
