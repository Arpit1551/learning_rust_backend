use std::sync::Mutex;

use actix_web::{
    App, Error, HttpResponse, HttpServer, Responder,
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    get,
    middleware::{Next, from_fn},
    post, web
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct UserCredentials {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[post("/login")]
async fn login(data: web::Json<UserCredentials>) -> impl Responder {
    let user_credentials = data.into_inner();
    println!("{:?}", user_credentials);

    if user_credentials.username == "Arpit" && user_credentials.password == "123" {
        let token = generate_token(user_credentials.username).await;
        return HttpResponse::Ok().body(format!("Login successfull !! Your token -> {:?}", token));
    }

    HttpResponse::NotFound().body(format!("Invalid login credentials !"))
}

#[get("get_counter")]
async fn get_counter(counter: web::Data<Mutex<i32>>) -> impl Responder {
    let counter = counter.lock().unwrap();
    HttpResponse::Ok().body(format!("Counter : {}", counter))
}

#[post("/add_counter")]
async fn add_counter(counter: web::Data<Mutex<i32>>) -> impl Responder {
    let mut counter = counter.lock().unwrap();
    *counter += 1;

    HttpResponse::Ok().body(format!("Counter : {}", counter))
}

async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    
    let auth_header = req.headers().get("Authorization").and_then(|x| x.to_str().ok());

    match auth_header {
        None => Ok(req.into_response(HttpResponse::Unauthorized().body("Unauthoried!!"))),

        Some(header_value) => {
            let token = header_value.replace("Bearer ", "");
            if verify_token(&token) {
                next.call(req).await
                    .map(|res| res.map_into_boxed_body())
            } else {
                Ok(req.into_response(HttpResponse::Unauthorized().body("Your token is expired!")))
            }
        }
    }
}

async fn generate_token(user_name: String) -> String {
    let claims = Claims {
        sub: user_name,
        exp: 9999999999,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("public".as_bytes()),
    )
    .unwrap()
}

fn verify_token(token: &String) -> bool {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret("public".as_bytes()),
        &Validation::default(),
    )
    .is_ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(Mutex::new(0));

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(web::scope("/public").service(login).service(get_counter))
            .service(
                web::scope("/private")
                .wrap(from_fn(auth_middleware))
                .service(add_counter))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
