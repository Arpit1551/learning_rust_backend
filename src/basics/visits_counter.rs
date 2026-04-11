// Create a visit counter that:
// GET /visits → returns how many times any route was visited
// GET /hello → increments counter + returns "Hello"
// GET /bye → increments counter + returns "Bye"


use std::sync::Mutex;

use actix_web::{ App, HttpResponse, HttpServer, Responder, get, web};

struct Visits {
    hello: u32,
    bye: u32
}

#[get("/hello")]
async fn hello(v: web::Data<Mutex<Visits>>) -> impl Responder {
    let mut visit = v.lock().unwrap();
    visit.hello += 1;

    HttpResponse::Ok().body("This is the hello page!")
}

#[get("/bye")]
async fn bye(v: web::Data<Mutex<Visits>>) -> impl Responder {
    let mut visit = v.lock().unwrap();
    visit.bye += 1;

    HttpResponse::Ok().body("This is the bye page!")
}

#[get("/visits")]
async fn visits_data(v: web::Data<Mutex<Visits>>) -> impl Responder {
    let visit = v.lock().unwrap();

    HttpResponse::Ok().body(format!("Visits on hello -> {}, Visits on bye -> {}", visit.hello, visit.bye))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let visit_data = web::Data::new(Mutex::new(Visits { hello:0, bye: 0 }));

    HttpServer::new( move || {
        App::new()
            .app_data(visit_data.clone())
            .service(hello)
            .service(bye)
            .service(visits_data)  
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}