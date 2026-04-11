// Create two scopes:
// /public → has GET /hello route, no state needed
// /admin → has its own separate state (an admin secret string) and GET /secret returns it

use std::sync::Mutex;

use actix_web::{ App, HttpResponse, HttpServer, Responder, get, web};

async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/show_secret")]
async fn show_secret( s: web::Data<Mutex<String>> ) -> impl Responder {

    let secret = s.lock().unwrap();
    HttpResponse::Ok().body(format!("Secret: {}" ,secret))

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret = web::Data::new(Mutex::new(String::from("This is the secret!")));

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/public")
                    .route("/hello", web::get().to(say_hello))
            )
            .service(
                web::scope("/admin")
                    .app_data(secret.clone())
                    .service(show_secret)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}