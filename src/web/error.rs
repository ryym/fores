use crate::error::Error;
use crate::prelude::*;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    messages: Vec<String>,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        log_error(&self);

        match self.kind() {
            ErrorKind::Db => error_res(StatusCode::INTERNAL_SERVER_ERROR, vec![self.to_string()]),
            ErrorKind::NotFound => error_res(StatusCode::NOT_FOUND, vec![self.to_string()]),
            ErrorKind::Invalid(msg) => error_res(StatusCode::BAD_REQUEST, vec![msg.clone()]),
            ErrorKind::Misc(msg) => error_res(StatusCode::INTERNAL_SERVER_ERROR, vec![msg.clone()]),
        }
    }
}

fn error_res(code: StatusCode, messages: Vec<String>) -> HttpResponse {
    HttpResponse::new(code)
        .into_builder()
        .json(ErrorResponse { messages })
}

fn log_error(err: &Error) {
    if let Some(c) = err.cause() {
        let causes = c
            .iter_chain()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        log::error!("ERROR: {}\nCAUSE: {}", err, causes);
    } else {
        log::error!("ERROR: {}", err);
    };
}
