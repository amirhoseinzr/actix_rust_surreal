mod database;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::{Namespace, Root};
use surrealdb::{Error, Surreal};
use surrealdb::iam::Level::No;
use crate::models::user::User;

#[derive(Clone)]
pub struct Database{
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self,Error> {

        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client.signin(Root {
            username: "root",
            password: "root",
        })
            .await?;
        client.use_ns("surreal").use_db("users").await.unwrap();
        Ok(Database{
            client,
            name_space: String::from("surreal"),
            db_name: String::from("users")
        })
    }
    //
    // pub async fn get_all_users(&self) -> Option<Vec<User>> {
    //     let result = self.client.select("user").await;  //here self refers to database
    //     match result {
    //         Ok(all_users) => Some(all_users),
    //         Err(_) => None,
    //     }
    // }
    // pub async fn add_user(&self, new_user: User) -> Option<User>{
    //     let created_user = self
    //         .client
    //         .create(("user", new_user.uuid.clone()))
    //         .content(new_user)
    //         .await;
    //
    //     match created_user {
    //         Ok(created) => created,
    //         Err(_) => None
    //     }
    // }

    // pub(crate)async fn update_user(&self, uuid:String) -> Option<User>{
    //     let find_user: Result<Option<User>, Error> = self.client.select(("user", &uuid)).await;
    //
    //     match find_user {
    //         Ok(found) => {
    //             match found {
    //                 Some(_found_user) => {
    //                       let updated_user: Result<Option<User>, Error> = self
    //                           .client
    //                           .update(("user", &uuid))
    //                           .merge(User {
    //                               uuid,
    //                               user_name: String::from(" Deleted")
    //                           })
    //                           .await;
    //                     match updated_user {
    //                         Ok(updated) => updated,
    //                         Err(_) => None,
    //                     }
    //                 },
    //                 None => None,
    //             }
    //         }
    //         Err (_) => None,
    //     }
    // }

}