// path: web::Path<u32>,           // from URL
// query: web::Query<SearchQuery>, // from query string
// body: web::Json<CreateUser>,    // from body
// state: web::Data<Mutex<T>>,     // from app state

use serde::Deserialize;
use actix_web::{ App, HttpResponse, HttpServer, Responder, get, web};

#[derive(Deserialize)]
struct Query{
    keyword: String,
    limit: u32
}

#[get("/test/{id}")] // Single param
async fn test(id: web::Path<u32>) -> impl Responder {
    let id = id.into_inner();
    HttpResponse::Ok().body(format!("This is the req from the id -> {}", id))
}

#[get("/test_two/{user_name}/{id}")] // multiple params
async fn test_two(path: web::Path<(String, u32)>) -> impl Responder {
    let (user_name, id) = path.into_inner();

    HttpResponse::Ok().body(format!("The user_name is {} ID is {}", user_name, id))
}

#[get("/search")]
async fn search(query: web::Query<Query>) -> impl Responder {
    HttpResponse::Ok().body(format!("The query keyword is -> {}, Limit is -> {}", query.keyword, query.limit))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( move || {
        App::new()
            .service(test)  
            .service(test_two)
            .service(search)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}