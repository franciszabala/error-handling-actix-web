use actix_web::dev::{Service};
use bcrypt::BcryptError;
use tokio::task::JoinError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse};
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {

    #[error("Error parsing ObjectID {0}")]
    ParseObjectID(String),

    #[error("{0}")]
    Authenticate(#[from] AuthenticateError),

    #[error("{0}")]
    BadRequest(#[from] BadRequest),

    #[error("{0}")]
    NotFound(#[from] NotFound),

    #[error("{0}")]
    RunSyncTask(#[from] JoinError),

    #[error("{0}")]
    HashPassword(#[from] BcryptError),
}

impl Error {

    //Added this function to get the status code from the enum
    pub fn get_status_code(&self) -> StatusCode {
        self.get_codes().0
    }

    fn get_codes(&self) -> (StatusCode, u16) {
        match *self {
            // 4XX Errors
            Error::ParseObjectID(_) => (StatusCode::BAD_REQUEST, 40001),
            Error::BadRequest(_) => (StatusCode::BAD_REQUEST, 40002),
            Error::NotFound(_) => (StatusCode::NOT_FOUND, 40003),
            Error::Authenticate(AuthenticateError::WrongCredentials) => (StatusCode::UNAUTHORIZED, 40004),
            Error::Authenticate(AuthenticateError::InvalidToken) => (StatusCode::UNAUTHORIZED, 40005),
            Error::Authenticate(AuthenticateError::Locked) => (StatusCode::LOCKED, 40006),

            // 5XX Errors
            Error::Authenticate(AuthenticateError::TokenCreation) => {
                (StatusCode::INTERNAL_SERVER_ERROR, 5001)
            }


            Error::RunSyncTask(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5005),
            Error::HashPassword(_) => (StatusCode::INTERNAL_SERVER_ERROR, 5006),
        }
    }


    pub fn bad_request(message : String) -> Self {
        Error::BadRequest(BadRequest {message})
    }

    pub fn not_found() -> Self {
        Error::NotFound(NotFound {})
    }
}


//I learned here that Rust allows you to have multiple chunks of impl {}
impl Error {
    pub fn error_response(&self) -> HttpResponse {
        let status_code = self.get_status_code();
        let message = self.to_string();
        let code = status_code.as_u16();
        let error_response = ErrorResponse { code, message };
        HttpResponse::build(status_code).json(error_response)
    }

    pub fn error_response_xml(&self) -> HttpResponse {
        let status_code = self.get_status_code();
        let message = self.to_string();
        let code = status_code.as_u16();
        let error_response = ErrorResponse { code, message };
        let error_resp_string = match to_string(&error_response) {
            Ok(xml_string) => xml_string,
            Err(_) => "<error>".to_string(), // Handle serialization error gracefully
        };
        HttpResponse::build(status_code).content_type("application/xml").body(error_resp_string)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,
    #[error("Failed to create authentication token")]
    TokenCreation,
    #[error("Invalid authentication credentials")]
    InvalidToken,
    #[error("User is locked")]
    Locked,
}

#[derive(thiserror::Error, Debug)]
#[error("Bad Request {message}")]
pub struct BadRequest {
    message : String
}

#[derive(thiserror::Error, Debug)]
#[error("Not found")]
pub struct NotFound {}

#[derive(Serialize,Deserialize)]
pub struct ErrorResponse {
    code : u16,
    message : String
}
