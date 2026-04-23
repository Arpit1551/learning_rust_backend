use actix_web::{HttpResponse, Responder, post, web};
use mongodb::Collection;
use mongodb::bson::doc;
use crate::models::user::{LoginUser, RegisterUser, User};
use crate::utils::jwt::generate_token;

#[post("/register")]
pub async fn register(
    db: web::Data<Collection<User>>,
    user_info: web::Json<RegisterUser>,
) -> impl Responder {
    match db.find_one(doc! {"email": &user_info.email}).await {
        Ok(Some(_)) => {
            return HttpResponse::BadRequest().body("User with email already exists!");
        }
        _ => {}
    }

    let new_user = User {
        id: None,
        username: user_info.username.clone(),
        email: user_info.email.clone(),
        password: user_info.password.clone(),
    };

    match db.insert_one(new_user).await {
        Ok(_) => {
            let token = generate_token(&user_info.email);
            HttpResponse::Ok().body(format!("Registered! Token -> {}", token))
        }
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong!"),
    }
}

#[post("/login")]
pub async fn login(
    db: web::Data<Collection<User>>,
    user_data: web::Json<LoginUser>,
) -> impl Responder {
    match db.find_one(doc! {"email": &user_data.email}).await {
        Ok(Some(user)) => {
            if user.password == user_data.password {
                let token = generate_token(&user.email);
                HttpResponse::Ok().body(format!("Login successful! Token -> {}", token))
            } else {
                HttpResponse::BadRequest().body("Wrong password!")
            }
        }
        Ok(None) => HttpResponse::NotFound().body("User not found!"),
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong!"),
    }
}