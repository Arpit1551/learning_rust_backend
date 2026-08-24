// use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Error};
// use actix_ws::Message;

// #[actix_web::main]
// async fn main() {
//     HttpServer::new(|| {
//         App::new()
//             .route("/ws", web::get().to(ws_handler))
//     })
//     .bind("127.0.0.1:7878").unwrap()
//     .run()
//     .await
//     .unwrap();
// }

// async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
//     let (res, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;
    
//     actix_rt::spawn(async move {
//         while let Some(Ok(msg)) = msg_stream.recv().await {
//             match msg {
//                 Message::Text(text) => {
//                     println!("Received: {}", text);
//                 }
//                 _ => {}
//             }
            
//         }
//     });

//     Ok(res)
// }

// Making a todo list app
// Deserialize → Client se data aana (incoming)
// Serialize → Client ko data jaana (outgoing)

