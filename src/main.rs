use std::fmt::format;
use actix_web::{get, patch, post, web::Path, web::Json, App, HttpResponse, HttpServer, Responder};
use surrealdb::sql::Data;
use validator::Validate;
mod models;
mod db;
use crate::db::Database;

use crate::models::user::{AddUserRequest, UpdateUserURL};


#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[get("/users")]
async fn get_users(db: Data<Database>) -> impl Responder{
    let users = db.get_all_usrs().await;

    match users {
        Some(found_users) => HttpResponse::Ok().body(format!("{:?}", found_users)),
        None => HttpResponse::Ok().body("Error"),
    }

    HttpResponse::Ok().body("Available Users")
}

#[post("/adduser")]
async fn add_user(body: Json<AddUserRequest>) -> impl Responder{
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let user_name = body.user_name.clone();
            HttpResponse::Ok().body(format!("user entered is: {user_name}" ,))
        }
        Err(_) => HttpResponse::Ok().body("user_name is required!")
    }
}


#[patch("/updateuser/{uuid}")]
async fn update_user(update_user_url: Path<UpdateUserURL>) -> impl Responder{
    let uuid = update_user.into_inner().uuid;
    HttpResponse::Ok().body(format!("updating user with {uuid}"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = Database::init()
        .await
        .expect("error to connect database");

    let db_data:Data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(get_users)
            .service(add_user)
            .service(update_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
