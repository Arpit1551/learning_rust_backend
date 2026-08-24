// The code is currently not working and also i am a bit confused in this topic!

// use actix_web::{App, HttpResponse, HttpServer, post};
// use actix_multipart::Multipart;
// use futures::{StreamExt, future::ok};

// #[post("/upload_file")]
// async fn upload_file(
//     mut payload: Multipart
// ) -> HttpResponse {

//     while let Some(field) = payload.next().await {

//         let mut field = match field {
//             Ok(f) => f,
//             Err(_) => return HttpResponse::BadRequest().body("Bad multipart data"),
//         };

//         let name = field.content_disposition().get_name().unwrap_or("").to_string();
//         if name != "file" {
//                 continue;
//         }

//         let mut bytes = Vec:new();
//         while let Some(chunk) = field.next().await {
//             let chunk = match chunk {
//                 Ok(c) => c,
//                 Err(_) => return HttpResponse::InternalServerError().body("Failed to read chunk"),
//             };
//             bytes.extend_from_slice(&chunk);
//         }

//         std::fs::create_dir_all("./uploads").ok();
//         let mut file = match std::fs::File::create("./uploads/myfile.bin") {
//             Ok(f) => f,
//             Err(_) => return HttpResponse::InternalServerError().body("Could not create file"),
//         };
//         if file.write_all(&bytes).is_err() {
//             return HttpResponse::InternalServerError().body("Could not write file");
//         }

//         return HttpResponse::Ok().body("File uploaded!");
//     }
//     HttpResponse::BadRequest().body("No field named 'file' found")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(move || {
//         App::new()
//             .service(upload_file)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

