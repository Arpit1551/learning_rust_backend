// Making a todo list app
// Deserialize → Client se data aana (incoming)
// Serialize → Client ko data jaana (outgoing)

use serde::Deserialize;
use std::sync::Mutex;
use actix_web::{ App, HttpResponse, HttpServer, Responder, get, post, web };

#[derive(Deserialize)]
struct TodoInput {
    task: String
}

#[get("/")]
async fn get_todos(todos: web::Data<Mutex<Vec<String>>>) -> impl Responder {
    let todos = todos.lock().unwrap();
    HttpResponse::Ok().body(format!("Todo -> {:?}", todos))
} 


#[post("/add_todo")]
async fn add_todo(todo: web::Json<TodoInput>, todos: web::Data<Mutex<Vec<String>>>) -> impl Responder {
    let new_todo = todo.task.clone();
    let mut todos: std::sync::MutexGuard<'_, Vec<String>> = todos.lock().unwrap();
    todos.push(new_todo);

    HttpResponse::Ok().body(format!(" The list of todos are -> {:?}",todos))
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let todos = web::Data::new(Mutex::new(vec![] as Vec<String>));

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