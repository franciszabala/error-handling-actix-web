use std::io::Read;
use actix_web::{App, HttpRequest, HttpServer, Responder, ResponseError, web};
use actix_web::body::MessageBody;
use crate::errors::{AuthenticateError, Error};

mod errors;


pub async fn send_authenticate_error(_req: HttpRequest) -> impl Responder {
    // This API will send auth error as response in JSON format
    Error::Authenticate(AuthenticateError::WrongCredentials).error_response()
}


// pub async fn send_bad_request(_req: HttpRequest) -> impl Responder {
//     // This API will send  bad request error as response inXMl format
//     Error::bad_request("Value is required".to_string()).error_response_xml()
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/auth_error", web::get().to(send_authenticate_error))
            //.route("/bad_request", web::get().to(send_bad_request))
    }).bind("0.0.0.0:8000")?
        .run()
        .await
}