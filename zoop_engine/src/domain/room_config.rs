use url::Url;
use zoop_shared::player_id::PlayerId;
use zoop_shared::room_id::RoomId;

#[derive(Clone, Debug)]
pub struct RoomConfig {
    pub server_address: Url,
    pub room: RoomId,
    pub user_id: PlayerId,
    pub user_ticket: String,
}
