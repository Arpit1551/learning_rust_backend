// Making a todo list app
// Deserialize → Client se data aana (incoming)
// Serialize → Client ko data jaana (outgoing)

use serde::Deserialize;
use actix_web::{ App, HttpResponse, HttpServer, Responder, get, post, web::{self, to} };
use dashmap::DashMap;

#[derive(Deserialize)]
struct TodoInput {
    task: String
}

#[get("/{usert_id}")]
async fn get_todos(user_id: web::Path<u32> ,app_sate: web::Data<DashMap<u32, Vec<String>>>) -> impl Responder {

    let user_id  = &user_id.into_inner();
    let todos = app_sate;

    if !todos.contains_key(user_id){
        return HttpResponse::Ok().body(format!("The user id {}, Does not have any todo!", user_id));
    }

    let todos = todos.get(user_id).unwrap();

    HttpResponse::Ok().body(format!("Todo -> {:?}", *todos))
} 


#[post("/add_todo/{user_id}")]
async fn add_todo(
    user_id: web::Path<u32>,
     todo: web::Json<TodoInput>,
      app_state: web::Data<DashMap<u32, Vec<String>>>
    ) -> impl Responder {

    let user_id = user_id.into_inner();
    let todo = todo.into_inner().task;
    app_state.entry(user_id).or_default().push(todo);

    HttpResponse::Ok().body("Your todos are updated!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let todos = web::Data::new(DashMap::<u32, Vec<String>>::new());
    
    HttpServer::new( move || {
        App::new()
            .app_data(todos.clone())
            .service(get_todos)
            .service(add_todo)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}