use url::Url;
use zoop_shared::room_id::RoomId;

#[derive(Clone, Debug)]
pub struct RoomConfig {
    pub server_address: Url,
    pub room: RoomId,
}
