use serde::{Deserialize, Serialize};
use validator::Validate;




#[derive(Deserialize, Serialize, Validate)]
pub struct AddUserRequest {
    #[validate(length(min = 1, message = "Username is required"))]
    pub user_name: String,
    pub wallet_address: String,

    // #[validate(email)]
    // pub email:String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateUserURL {
    pub uuid: String,
}

#[derive(Serialize,Deserialize, Validate, Debug)]
pub struct User{
    pub uuid: String,
    pub user_name: String,
    pub wallet_address: String,
}

impl User{
    pub fn new(uuid: String, user_name: String, wallet_address: String) -> User {
        User {uuid , user_name, wallet_address}
    }
}