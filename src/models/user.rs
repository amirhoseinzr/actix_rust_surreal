use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct AddUserRequest {
    #[validate(length(min = 1, message = "Username is required"))]
    pub user_name: String,
}
