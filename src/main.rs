
use actix_web::{get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
use validator::Validate;
mod models;
use models::AddUserRequest;


#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[get("/user")]
async fn get_users() -> impl Responder{
    HttpResponse::Ok().body("Available Users")
}

#[post("/adduser")]
async fn add_user(body: Json<AddUserRequest>) -> impl Responder{
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let user_name = body.user_name.clone();
            HttpResponse::Ok().body(format!("user entered{user_name}" ,))
        }
        Err(_) => HttpResponse::Ok().body("user_name is required!")
    }
}

#[patch("/updateuser/{uuid}")]
async fn update_user(body: Json<AddUserRequest>) -> impl Responder{
    HttpResponse::Ok().body("pach func")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_users)
            .service(add_user)
            .service(update_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
