use crate::actors::player::GamePlayer;
use actix::*;

use std::collections::HashMap;
use zoop_shared::player_id::PlayerId;
use zoop_shared::player_message::PlayerMessage;
use zoop_shared::room_id::RoomId;

/// Game room comms
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Ping;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Register {
    pub id: PlayerId,
    pub address: Addr<GamePlayer>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FromPlayer {
    pub message: PlayerMessage,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct FromToPlayer {
    pub from: PlayerId,
    pub message: PlayerMessage,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ToPlayer {
    pub message: PlayerMessage,
}

/// Define game room
pub struct GameRoom {
    id: RoomId,
    player_count: u32,
    players: HashMap<PlayerId, Addr<GamePlayer>>,
}
impl GameRoom {
    fn add_player(&mut self, id: PlayerId, address: Addr<GamePlayer>) -> Option<Addr<GamePlayer>> {
        self.players.insert(id, address)
    }

    pub fn of(id: RoomId, player_count: u32) -> GameRoom {
        GameRoom {
            id,
            player_count,
            players: HashMap::new(),
        }
    }
}
impl Actor for GameRoom {
    type Context = Context<Self>;
}
impl Handler<Ping> for GameRoom {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, _msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");
        Ok(true)
    }
}
impl Handler<Register> for GameRoom {
    type Result = ();

    fn handle(&mut self, register: Register, _ctx: &mut Context<Self>) -> Self::Result {
        let _ = self.add_player(register.id.clone(), register.address);
        println!("Room {} accepted player {}", self.id, register.id);
    }
}
impl Handler<FromToPlayer> for GameRoom {
    type Result = ();

    fn handle(&mut self, from_to: FromToPlayer, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(address) = self.players.get(&from_to.message.address) {
            let should_drop: bool;
            #[cfg(feature = "drop_messages")]
            {
                let mut rng = rand::thread_rng();
                let rand = rng.gen_range(0.0..1.0);
                should_drop = rand < 0.05; // 5% chance of dropping package
            }
            #[cfg(not(feature = "drop_messages"))]
            {
                should_drop = false;
            }

            if !should_drop {
                let _ = address.try_send(FromPlayer {
                    message: PlayerMessage::from(from_to.from, from_to.message.message),
                });
            }
        }
        ()
    }
}
