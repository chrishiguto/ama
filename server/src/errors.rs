use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use wither::mongodb::error::Error as MongoError;
use wither::WitherError;

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    ParseObjectID(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InternalServerError(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    Wither(#[from] WitherError),

    #[error("{0}")]
    Mongo(#[from] MongoError),
}

impl Error {
    pub fn bad_request(message: String) -> Self {
        Error::BadRequest(message)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::ParseObjectID(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Wither(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Mongo(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let status_code = self.status_code();

        let error_message = ErrorResponse {
            status: status_code.into(),
            error: self.to_string(),
        };

        HttpResponse::build(status_code).json(error_message)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request")]
pub struct BadRequest {}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    status: u16,
    error: String,
}
