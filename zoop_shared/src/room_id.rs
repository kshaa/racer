use derive_more::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Display, Eq, PartialEq, Hash)]
pub struct RoomId(pub Uuid);
impl RoomId {
    pub fn new() -> RoomId {
        RoomId(Uuid::new_v4())
    }
}
