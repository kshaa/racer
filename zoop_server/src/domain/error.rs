use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web_actors::ws::{CloseCode, CloseReason};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use zoop_shared::player_id::PlayerId;
use zoop_shared::room_id::RoomId;

#[derive(Serialize, Deserialize, Debug, derive_more::Error)]
pub struct WrappedError {
    error: AppError,
    message: String,
}
impl Display for WrappedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.error)
    }
}

/// Server errors
#[derive(Serialize, Deserialize, Debug, Display, derive_more::Error, Clone)]
pub enum AppError {
    #[display(fmt = "Invalid user ticket, unauthorized")]
    UserTicketWrong(),
    #[display(fmt = "A room requires at least 2 players")]
    NotEnoughPlayers(),
    #[display(fmt = "User with id {} already exists", id.value)]
    UserAlreadyExists { id: PlayerId },
    #[display(fmt = "User with name '{}' already exists", username)]
    UsernameAlreadyExists { username: String },
    #[display(fmt = "Username must be alphanumeric: '{}'", username)]
    NotAlphanumericUsername { username: String },
    #[display(
        fmt = "Username must be no longer than {} symbols: '{}'",
        max_length,
        username
    )]
    TooLongUsername { max_length: u32, username: String },
    #[display(fmt = "Game with id {} already exists", id.value)]
    GameAlreadyExists { id: RoomId },
    #[display(fmt = "Game with id {} does not exist", id.value)]
    GameDoesNotExist { id: RoomId },
    #[display(fmt = "Only the game creator can alter game config")]
    NotGameCreator(),
    #[display(fmt = "Game not started yet")]
    GameNotReady(),
    #[display(fmt = "Room failed to respond to player registration")]
    RoomNotResponding(),
    #[display(fmt = "Room is already full of players")]
    RoomFull(),
    #[display(fmt = "Unrecognized or bad message received")]
    BadMessage(),
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::UserTicketWrong { .. } => StatusCode::UNAUTHORIZED,
            AppError::NotEnoughPlayers { .. } => StatusCode::BAD_REQUEST,
            AppError::UserAlreadyExists { .. } => StatusCode::BAD_REQUEST,
            AppError::UsernameAlreadyExists { .. } => StatusCode::BAD_REQUEST,
            AppError::NotAlphanumericUsername { .. } => StatusCode::BAD_REQUEST,
            AppError::TooLongUsername { .. } => StatusCode::BAD_REQUEST,
            AppError::GameAlreadyExists { .. } => StatusCode::BAD_REQUEST,
            AppError::GameDoesNotExist { .. } => StatusCode::BAD_REQUEST,
            AppError::NotGameCreator { .. } => StatusCode::BAD_REQUEST,
            AppError::GameNotReady { .. } => StatusCode::NOT_FOUND,
            AppError::RoomNotResponding { .. } => StatusCode::BAD_GATEWAY,
            AppError::RoomFull { .. } => StatusCode::BAD_REQUEST,
            AppError::BadMessage { .. } => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let wrapped = WrappedError {
            error: self.clone(),
            message: format!("{}", self),
        };
        HttpResponse::build(self.status_code()).json(wrapped)
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
