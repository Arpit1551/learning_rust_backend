use actix_web::{HttpRequest, HttpResponse, Responder, HttpMessage, delete, get, put, web, middleware::from_fn};
use mongodb::Collection;
use mongodb::bson::doc;
use crate::models::user::{UpdatedUser, User};
use crate::utils::jwt::Claims;
use crate::middleware::auth::auth_middleware;

#[get("/profile")]
pub async fn profile(
    db: web::Data<Collection<User>>,
    req: HttpRequest,
) -> impl Responder {
    let ext = req.extensions();
    let claims = ext.get::<Claims>();

    match claims {
        Some(claims) => {
            match db.find_one(doc! {"email": &claims.email}).await {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::NotFound().body("User not found!"),
            }
        }
        None => HttpResponse::Unauthorized().body("No claims found!"),
    }
}

#[put("/update")]
pub async fn update_user(
    db: web::Data<Collection<User>>,
    req: HttpRequest,
    user_data: web::Json<UpdatedUser>,
) -> impl Responder {
    let ext = req.extensions();
    let claims = ext.get::<Claims>();

    match claims {
        Some(claims) => {
            match db.update_one(
                doc! {"email": &claims.email},
                doc! {"$set": {
                    "username": &user_data.username,
                    "password": &user_data.password
                }},
            ).await {
                Ok(_) => HttpResponse::Ok().body("User updated!"),
                Err(_) => HttpResponse::InternalServerError().body("Something went wrong!"),
            }
        }
        None => HttpResponse::Unauthorized().body("No claims found!"),
    }
}

#[delete("/delete")]
pub async fn delete_user(
    db: web::Data<Collection<User>>,
    req: HttpRequest,
) -> impl Responder {
    let ext = req.extensions();
    let claims = ext.get::<Claims>();

    match claims {
        Some(claims) => {
            match db.delete_one(doc! {"email": &claims.email}).await {
                Ok(_) => HttpResponse::Ok().body("User deleted!"),
                Err(_) => HttpResponse::InternalServerError().body("Something went wrong!"),
            }
        }
        None => HttpResponse::Unauthorized().body("No claims found!"),
    }
}

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .wrap(from_fn(auth_middleware))
                .service(profile)
                .service(update_user)
                .service(delete_user)
    );
}