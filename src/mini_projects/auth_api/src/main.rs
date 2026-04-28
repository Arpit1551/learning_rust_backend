mod db;
mod middleware;
mod models;
mod routes;
mod utils;

use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use actix_web::http::header;
use actix_governor::{ Governor, GovernorConfigBuilder };

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    
    let collection = db::connect_db().await;
    let db = web::Data::new(collection);
    
    HttpServer::new(move || {
        
        let governer_config = GovernorConfigBuilder::default()
                                                    .per_second(2)
                                                    .burst_size(5)
                                                    .finish()
                                                    .unwrap();

        let cors = Cors::default()
                        .allowed_origin("https://localhost:3000")
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                        .max_age(3600);

        App::new()
            .wrap(Governor::new(&governer_config))
            .wrap(cors)
            .app_data(db.clone())
            .configure(routes::auth::auth_conifg)
            .configure(routes::user::user_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}