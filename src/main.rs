use actix_web::{
    App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, body::{BoxBody, MessageBody}, delete, dev::{ServiceRequest, ServiceResponse}, get, middleware::{Next, from_fn}, post, put, web
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use mongodb::{
    Client, Collection,
    bson::{doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    email: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct UpdatedUser {
    username: String,
    password: String
}

#[post("/register")]
async fn register(
    db: web::Data<Collection<User>>,
    user_info: web::Json<RegisterUser>,
) -> impl Responder {
    let new_user = User {
        id: None,
        username: user_info.username.clone(),
        email: user_info.email.clone(),
        password: user_info.password.clone(),
    };

    let user_exists;
    match db.find_one(doc! {"email": &new_user.email}).await {
        Ok(Some(_)) => user_exists = true,
        Ok(None) => user_exists = false,
        Err(_) => user_exists = false,
    }

    if user_exists {
        HttpResponse::BadRequest().body("User with email already exist!")
    } else {
        match db.insert_one(new_user).await {
            Ok(_) => {
                let token = generate_token(&user_info.email);
                HttpResponse::Ok().body(format!(
                    "User created successfully!, Your Token -> {}",
                    token
                ))
            }
            Err(_) => HttpResponse::BadRequest().body("InternalServerError!"),
        }
    }
}

#[post("/login")]
async fn login(db: web::Data<Collection<User>>, user_data: web::Json<LoginUser>) -> impl Responder {
    match db.find_one(doc! {"email": user_data.email.clone()}).await {
        Ok(Some(user_info)) => {
            if user_info.password == user_data.password {
                let token = generate_token(&user_info.email);
                HttpResponse::Ok().json(json!({
                    "user_info": user_info,
                    "token": token
                }))
            } else {
                HttpResponse::BadRequest().body("wrong password!")
            }
        }
        Ok(None) => HttpResponse::NotFound().body(format!("User does not exist!")),
        Err(_) => HttpResponse::NotFound().body(format!("User does not exist!")),
    }
}

async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let auth_head = req
        .headers()
        .get("Authorization")
        .and_then(|x| x.to_str().ok());

    match auth_head {
        None => Ok(req.into_response(HttpResponse::Unauthorized().body("Unauthorized!"))),
        Some(header_value) => {
            let token = header_value.replace("Bearer ", "");
            match decode_token(&token) {
                Some(claims) => {
                    req.extensions_mut().insert(claims);
                    next.call(req).await.map(|res| res.map_into_boxed_body())
                },
                None => Ok(req.into_response(HttpResponse::Unauthorized().body("Invalid token!"))),
            }
        }
    }
}

#[get("/profile")]
async fn profile(db: web::Data<Collection<User>>, req: HttpRequest) -> impl Responder {

    let extension = req.extensions();
    let claims = extension.get::<Claims>();

    match claims {
        Some(claims) => {
            match db.find_one(doc! {"email": &claims.email}).await {
                Ok(user) => {
                    HttpResponse::Ok().json(user)
                },
                Err(_) => {
                    HttpResponse::Unauthorized().body("User NotFound!")
                }
            }
        },
        None => {
            HttpResponse::Unauthorized().body("No claims found")
        }
    }
}

#[put("/update")]
async fn update_user(
    db: web::Data<Collection<User>>, 
    req: HttpRequest, 
    user_data: web::Json<UpdatedUser>
) -> impl Responder{
    
    let extension = req.extensions();
    let claims = extension.get::<Claims>();
    
    match claims {
        Some(claim) => {
            match db.update_one(
                    doc! {
                        "email": &claim.email
                    },
                    doc! {
                        "$set": {
                            "username": &user_data.username,
                            "password": &user_data.password
                    }
                }
            ).await {
                Ok(res) => {
                    HttpResponse::Ok().json(res)
                },
                Err(_) => {
                    HttpResponse::BadRequest().body("Somethign went wrong!")
                }
            }
        },
        None => {
            HttpResponse::Unauthorized().body("No claims found")
        }
    }
}

#[delete("/delete")]
async fn delete_user(db: web::Data<Collection<User>>, req: HttpRequest) -> impl Responder {

    let extension = req.extensions();
    let claims = extension.get::<Claims>();
    
    match claims {
        Some(claims) => {
            match db.delete_one(doc! {"email": &claims.email}).await {
                Ok(res) => {
                    HttpResponse::Ok().body(format!("User deleted! -> {:?}",res))
                },
                Err(_) => {
                    HttpResponse::BadRequest().body("Somethign went wrong!")
                }
            }
        },
        None => {
            HttpResponse::Unauthorized().body("No claims found")
        }
    }

}

async fn connect_db() -> Collection<User> {
    let db = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    db.database("mydb").collection::<User>("users")
}

fn generate_token(email: &String) -> String {

    let exp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as usize + (24 * 60 * 60);

    let claims = Claims {
        email: email.clone(),
        exp: exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("change_in_production".as_bytes()),
    )
    .unwrap()
}

fn decode_token(token: &String) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("change_in_production".as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let collection = connect_db().await;
    let db = web::Data::new(collection);

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(web::scope("/auth").service(register).service(login))
            .service(
                web::scope("/user")
                    .wrap(from_fn(auth_middleware))
                    .service(profile)
                    .service(update_user)
                    .service(delete_user)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
