use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use meilisearch_sdk::{client::*, document::*, search::*};

mod db;

#[get("/api/{word}")]
async fn search_word(web::Path(word): web::Path<String>) -> impl Responder {
    let response: Vec<SearchResult<db::Word>> = db::search(word);
    HttpResponse::Ok().body(format!("Hello {:#?}!", response))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(search_word)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
