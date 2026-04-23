mod db;
mod middleware;
mod models;
mod routes;
mod utils;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::from_fn;
use middleware::auth::auth_middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let collection = db::connect_db().await;
    let db = web::Data::new(collection);

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(
                web::scope("/auth")
                    .service(routes::auth::register)
                    .service(routes::auth::login)
            )
            .service(
                web::scope("/user")
                    .wrap(from_fn(auth_middleware))
                    .service(routes::user::profile)
                    .service(routes::user::update_user)
                    .service(routes::user::delete_user)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}