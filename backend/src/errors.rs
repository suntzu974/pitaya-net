use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use mobc_postgres::tokio_postgres;

use std::convert::From;
use uuid::Error as ParseError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "WrongEmailOrPassword")]
    WrongEmailOrPassword,


}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::WrongEmailOrPassword => HttpResponse::Unauthorized().json("Wrond email or password"),

        }
    }
}

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}


impl From<lettre::smtp::error::Error> for ServiceError {
    fn from(_: lettre::smtp::error::Error) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<deadpool_postgres::PoolError> for ServiceError {
    fn from(_: deadpool_postgres::PoolError) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<tokio_postgres::error::Error> for ServiceError {
    fn from(_: tokio_postgres::error::Error) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<argon2::Error> for ServiceError {
    fn from(_: argon2::Error) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<tokio_pg_mapper::Error> for ServiceError {
    fn from(_: tokio_pg_mapper::Error) -> Self {
        ServiceError::InternalServerError
    }
}
