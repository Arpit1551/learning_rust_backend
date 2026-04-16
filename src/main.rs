use std::path::Path;

use actix_web::{ App, HttpResponse, HttpServer, Responder, get, post, web::{self} };
use serde::{Deserialize, Serialize};
use mongodb::{ Client, Collection, bson::{doc, oid::ObjectId} };
use futures::TryStreamExt;

#[derive(Debug, Serialize,  Deserialize)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    username: String,
    password: String
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    username: String,
    password: String
}

#[get("/")]
async fn landing() -> impl Responder {
    HttpResponse::Ok().body(format!("Working!"))
}

#[post("/set_data")]
async fn set_data( db: web::Data<Collection<User>> ,data: web::Json<UserInfo>) -> impl Responder {
    let user_data = data.into_inner();
    
    let new_user = User {
        id: None,
        username: user_data.username,
        password: user_data.password
    };

    match db.insert_one(new_user).await {
        Ok(result) => HttpResponse::Ok().body(format!("Data inserted successfully. ID -> {}", result.inserted_id)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e))
    }
}

#[get("/get_users")]
async fn get_users(db: web::Data<Collection<User>>) -> impl Responder{
    match db.find(doc! {}).await {
        Ok(users) => {
            let users: Vec<User> = users.try_collect().await.unwrap_or_default();
            HttpResponse::Ok().json(users)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    } 
}

#[get("/get_user/{id}")]
async fn get_user( id: web::Path<String>, db: web::Data<Collection<User>> ) -> impl Responder {

    match ObjectId::parse_str(id.into_inner()) {
        Err(_) => {
            HttpResponse::BadRequest().body("Invalid id!")
        },
        Ok(obj_id) => {
            match db.find_one(doc! {"_id": obj_id}).await {
                Ok(user) => {
                    HttpResponse::Ok().json(user)
                },
                Err(e) => {
                    HttpResponse::NotFound().body("User not found!")
                }
            }
        }
    }

}

#[post("delete_user/{id}")]
async fn delete_user( user_id: web::Path<String>, db: web::Data<Collection<User>> ) -> impl Responder {

    match ObjectId::parse_str( user_id.into_inner() ) {

        Err(_) => {
            HttpResponse::BadRequest().body("Invalid ID!")
        },
        Ok(obj_id) => {
            match db.delete_one(doc! {"_id": obj_id}).await {
                Err(e) => {
                    HttpResponse::InternalServerError().body(format!("Something went wrong -> {:?}",e))
                },
                Ok(_) => {
                    HttpResponse::Ok().body(format!("User deleted successfully"))
                }
            }
        }

    }
}

async fn connect_db() -> Collection<User> {
    let db = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    db.database("mydb").collection::<User>("users")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let collection = connect_db().await;
    let db = web::Data::new(collection);

    HttpServer::new( move || {
        App::new()
            .app_data(db.clone())
            .service(landing)
            .service(set_data)
            .service(get_users)
            .service(get_user)
            .service(delete_user)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}