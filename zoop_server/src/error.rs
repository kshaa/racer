use actix::MailboxError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web_actors::ws::{CloseCode, CloseReason};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use zoop_shared::room_id::RoomId;

/// Server errors
#[derive(Serialize, Deserialize, Debug, Display, derive_more::Error)]
pub enum AppError {
    #[display(fmt = "Game with id {} already exists", id.value)]
    GameAlreadyExists { id: RoomId },
    #[display(fmt = "Game with id {} does not exist", id.value)]
    GameDoesNotExist { id: RoomId },
    #[display(fmt = "Room failed to respond to player registration")]
    RoomNotResponding(),
    #[display(fmt = "Unrecognized or bad message received")]
    BadMessage(),
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::GameAlreadyExists { .. } => StatusCode::BAD_REQUEST,
            AppError::GameDoesNotExist { .. } => StatusCode::BAD_REQUEST,
            AppError::RoomNotResponding { .. } => StatusCode::BAD_GATEWAY,
            AppError::BadMessage { .. } => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl AppError {
    pub fn close_reason(&self) -> CloseReason {
        CloseReason {
            code: CloseCode::Policy,
            description: Some(format!("{}", self)),
        }
    }
}
