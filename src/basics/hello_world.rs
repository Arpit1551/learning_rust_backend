use actix_web::{App, HttpResponse, HttpServer, Responder, get, web::Json};

#[get("/")]
async fn landing() -> impl Responder {
    HttpResponse::Ok.body(format!("Hello world!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( move || {
        App::new()
            .service(landing)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
