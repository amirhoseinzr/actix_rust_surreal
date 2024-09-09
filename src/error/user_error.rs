use actix_web::{http::{header::ContentType, StatusCode}, HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use derive_more::Display;
use surrealdb::error::Db::Internal;

#[derive(Debug,Display)]
pub enum UserError{
    NotUserFound,
    UsercreationFailed,
    NoSuchUser,
}

impl ResponseError for UserError{
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::NotUserFound => StatusCode::NOT_FOUND,
            UserError::UsercreationFailed => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::NoSuchUser => StatusCode::NOT_FOUND,
        }
    }
}