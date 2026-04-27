mod db;
mod middleware;
mod models;
mod routes;
mod utils;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let collection = db::connect_db().await;
    let db = web::Data::new(collection);

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .configure(routes::auth::auth_conifg)
            .configure(routes::user::user_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}