use std::fmt::format;
use actix_web::{get, patch, post, web::Path, web::Json, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use surrealdb::opt::QueryResult;
use surrealdb::sql::Statement::Use;
use validator::Validate;
mod models;
use crate::db::user_data_trait::{UserDataTrait};
use crate::db::Database;

mod db;
use crate::models::user::{AddUserRequest, UpdateUserURL, User };
use surrealdb::sql::Uuid;
use crate::error::user_error::UserError;
mod error  ;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[get("/users")]
async fn get_users(db: Data<Database>) -> Result<Json<Vec<User>> , UserError>{

    let users = Database::get_all_users(&db).await;

    match users {
        Some(found_users) =>Ok(Json(found_users)),
        None => Err(UserError::NoSuchUser),
    }

    //HttpResponse::Ok().body("Available Users")
}

// #[post("/adduser")]
// async fn add_user(body: Json<AddUserRequest>, db:  Data<Database>) -> impl Responder{
//     let is_valid = body.validate();
//     match is_valid {
//         Ok(_) => {
//             let user_name = body.user_name.clone();
//             let mut buffer = uuid::Uuid::encode_buffer();
//             let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
//             let new_user = db
//                 .add_user(User::new(String::from(new_uuid), user_name))
//                 .await;
//             match new_user {
//                 Some(created) => {
//                     return HttpResponse::Ok().body(format!("created new usr: {:?}", created))
//                 },
//                 None => return HttpResponse::Ok().body("Error to add user"),
//             }
//         } Err(_) => return HttpResponse::Ok().body("user_name is required!")
//     }
// }

#[post("/adduser")]
async fn add_user(body: Json<AddUserRequest>, db: Data<Database>) -> Result<Json<User>, UserError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let user_name = body.user_name.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_user = Database::
                add_user( &db,User::new(String::from(new_uuid), user_name))
                .await;

            match new_user {
                Some(created) => {
                    // Return the response, no semicolon here
                    Ok(Json(created))
                },
                None => {
                    Err(UserError::UsercreationFailed)
                }
            }
        }
        Err(_) => {
            Err(UserError::UsercreationFailed)
        }
    }
}


#[patch("/updateuser/{uuid}")]
async fn update_user(update_user_url: Path<UpdateUserURL>, db:Data<Database>) -> Result<Json<User>, UserError>  {
    let uuid = update_user_url.into_inner().uuid;
    let update_result = Database::update_user(&db, uuid).await;

    match update_result {
        Some(updated_user) => Ok(Json(updated_user)),
        None => Err(UserError::NoSuchUser),

    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = Database::init()
        .await
        .expect("error to connect database");

    let db_data  = Data::new(db);

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
