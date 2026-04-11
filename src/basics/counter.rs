use std::sync::Mutex;

use actix_web::{ App, HttpResponse, HttpServer, Responder, get, post, web };

#[get("/")]
async fn check_counter(counter: web::Data<Mutex<i32>>) -> impl Responder {
    let val = counter.lock().unwrap();

    HttpResponse::Ok().body(format!("Counter: {}", *val))
}

#[post("/")]
async fn increment_counter(counter: web::Data<Mutex<i32>>) -> impl Responder {
    let mut val = counter.lock().unwrap();
    *val += 1;

    HttpResponse::Ok().body(format!("Counter: {}", *val))
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let counter = web::Data::new(Mutex::new(0));

    HttpServer::new( move || {
        App::new()
            .app_data(counter.clone())
            .service(check_counter)
            .service(increment_counter)
    })
        .bind(("127.0.0.1",8080))?
        .run()
        .await
}