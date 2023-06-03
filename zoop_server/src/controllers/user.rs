use crate::actors::room::*;
use crate::domain::error::*;
use crate::domain::users::{User, Users};
use actix::*;
use actix_web::web;
use actix_web::web::Data;
use actix_web::{post, Result};
use std::sync::Mutex;

struct Registration {
    pub username: String,
}

#[post("/user/register/{username}")]
pub async fn user_create(
    path: web::Path<String>,
    users_mutex: Data<Mutex<Users>>,
) -> Result<web::Json<User>, AppError> {
    let username = path.as_ref();
    let mut users = users_mutex.lock().unwrap();
    users
        .add(username.to_owned())
        .map(|user| {
            println!("New user '{}'", username);
            user
        })
        .map(|user| web::Json(user))
}
