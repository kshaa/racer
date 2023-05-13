use derive_more::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Display, Eq, PartialEq, Hash)]
pub struct PlayerId(pub Uuid);
impl PlayerId {
    pub fn new() -> PlayerId {
        PlayerId(Uuid::new_v4())
    }
}