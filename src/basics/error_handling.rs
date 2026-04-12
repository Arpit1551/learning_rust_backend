// errors

// Handling the errors with inbuild actix web erros

use actix_web::{App, HttpResponse, HttpServer, Responder, ResponseError, get, web};
use std::fmt;

// #[get("/user/{id}")]
// async fn get_user(id: web::Path<u32>) -> impl Responder {
//     let id = id.into_inner();

//     if id == 0 {
//         return HttpResponse::NotFound().body("ID cannot be 0")
//     };

//     HttpResponse::Ok().body(format!("This is the req from {}", id))
// }

// #[get("/divide/{a}/{b}")]
// async fn divide(params: web::Path<(u32, u32)>) -> impl Responder {
//     let (a, b) = params.into_inner();

//     if b == 0 {
//        return HttpResponse::BadRequest().body("Cannot divid by 0");
//     } 

//     HttpResponse::Ok().body(format!("{} / {} = {}", a, b, a/b))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(move || {
//         App::new()
//             .service(get_user)  
//             .service(divide)
//     })
//     .bind(("127.0.0.1",8080))?
//     .run()
//     .await
// }

// Handling errors with custom errors

#[derive(Debug)]
enum AppErr {
    NotFound(String),
    BadRequest(String)
}

impl fmt::Display for AppErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErr::NotFound(msg) => write!(f, "Not found : {}",msg),
            AppErr::BadRequest(msg) => write!(f, "Bad Request : {}",msg),
        }
    }
}

impl ResponseError for AppErr {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppErr::NotFound(msg) => HttpResponse::NotFound().body(msg.clone()),
            AppErr::BadRequest(msg) => HttpResponse::BadRequest().body(msg.clone())
        }
    }
}

#[get("/user/{id}")]
async fn get_user(id: web::Path<u32>) -> Result<impl Responder, AppErr> {
    let id = id.into_inner();

    if id == 0 {
        return Err(AppErr::NotFound("Id can never be 0".to_string()));
    }

    Ok(HttpResponse::Ok().body(format!("The id of the user is {}", id)))
}

#[get("/divide/{a}/{b}")]
async fn divide(params: web::Path<(u32, u32)>) -> Result<impl Responder, AppErr> {
    let (a, b) = params.into_inner();

    if b == 0 {
        return Err(AppErr::BadRequest("Cannot divide by zero".to_string()));
    }

    Ok(HttpResponse::Ok().body(format!("{} / {} = {}", a, b, a/b)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( move || {
        App::new()
            .service(get_user)
            .service(divide)  
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}