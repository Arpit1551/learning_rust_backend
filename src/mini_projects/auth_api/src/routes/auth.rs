use crate::models::user::{LoginUser, RegisterUser, User};
use crate::utils::{
    jwt::generate_token,
    password::{hash_password, verify_password},
};
use actix_web::{HttpResponse, Responder, post, web};
use mongodb::Collection;
use mongodb::bson::doc;

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

    let password = match hash_password(&user_info.password) {
        Ok(h) => h,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to hash password!");
        }
    };

    let new_user = User {
        id: None,
        username: user_info.username.clone(),
        email: user_info.email.clone(),
        password: password,
    };

    match db.insert_one(new_user).await {
        Ok(_) => {
            let token = generate_token(&user_info.email);
            HttpResponse::Created().json(serde_json::json!({
                "message": "Registration successful!",
                "token": token
            }))
        }
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong!"),
    }
}

#[post("/login")]
pub async fn login(
    db: web::Data<Collection<User>>,
    user_data: web::Json<LoginUser>,
) -> impl Responder {
    let user = match db.find_one(doc! {"email": &user_data.email}).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::NotFound().body("User not found!");
        }
        Err(_) => {
            return HttpResponse::InternalServerError().body("Something went wrong!");
        }
    };

    match verify_password(&user_data.password, &user.password) {
        Ok(true) => {
            let token = generate_token(&user.email);
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Login successful!",
                "token": token
            }))
        }
        Ok(false) => HttpResponse::Unauthorized().body("Invalid email or password."),
        Err(_) => HttpResponse::BadRequest().body("Wrong password!"),
    }
}

pub fn auth_conifg(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(register).service(login));
}
