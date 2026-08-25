use actix_web::{ HttpResponse, HttpServer, App, get, web, Responder };
use sqlx::postgres::PgPoolOptions;

#[get("/")]
async fn landing() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the landing page!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Database url must be set");
        let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    println!("Database connected!");

    HttpServer::new( move ||
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(landing)  
    )
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
