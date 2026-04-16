use actix_web::{ App, HttpResponse, HttpServer, Responder, get, post, web::{self, Data} };
use serde::{Deserialize, Serialize};
use mongodb::{ Client, Collection, bson::{doc, oid::ObjectId} };

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
    match db.find_one( doc! {} ).await {
        Ok(users) => {
            HttpResponse::Ok().body(format!("User info {:?}", users))
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
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
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}