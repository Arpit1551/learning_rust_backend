use actix_web::{ App, HttpServer, HttpResponse, Responder, web, post };
use sqlx::{PgPool, postgres::PgPoolOptions};

#[post("/get_users")]
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    
    let users = sqlx::query("Select * from users")
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

        HttpResponse::Ok().body(format!("User data {:?}", users))

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("Database url must be set!");
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&db_url)
    .await
    .expect("Database connection failed!");

    println!("Database connected!");

    HttpServer::new( move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(get_users)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}