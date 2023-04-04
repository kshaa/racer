use crate::error::AppError;
use crate::misc::*;
use crate::room::*;
use actix::fut::Ready;
use actix::*;
use actix_web_actors::ws;
use shared::{PlayerId, PlayerMessage, RoomId};

/// Define game player
pub struct GamePlayer {
    pub player_id: PlayerId,
    pub room_id: RoomId,
    pub room_address: Addr<GameRoom>,
}
impl GamePlayer {
    fn handle_registration(
        res: actix_web::Result<(), MailboxError>,
        act: &mut GamePlayer,
        ctx: &mut ws::WebsocketContext<Self>,
    ) -> Ready<()> {
        match res {
            Ok(()) => println!(
                "Player {} got place in room {}",
                &act.player_id.clone(),
                &act.room_id.clone()
            ),
            Err(mailbox_error) => {
                let app_error = AppError::RoomNotResponding();
                println!(
                    "Room {} failed to register player {}: {}",
                    &act.room_id.clone(),
                    &act.player_id.clone(),
                    mailbox_error
                );
                ctx.text(serde_json::to_string(&app_error).unwrap());
                ctx.close(Some(app_error.close_reason()));
            }
        };
        fut::ready(())
    }
}
impl Handler<ToPlayer> for GamePlayer {
    type Result = ();
    fn handle(&mut self, to: ToPlayer, ctx: &mut ws::WebsocketContext<Self>) -> Self::Result {
        let _ = self.room_address.try_send(FromToPlayer {
            from: self.player_id.clone(),
            message: to.message,
        });
        ()
    }
}
impl Handler<FromPlayer> for GamePlayer {
    type Result = ();
    fn handle(&mut self, from: FromPlayer, ctx: &mut ws::WebsocketContext<Self>) -> Self::Result {
        ctx.text(serde_json::to_string(&from.message).unwrap());
        ()
    }
}
impl Actor for GamePlayer {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Player {} joining room {}", &self.player_id, &self.room_id);
        let request = self.room_address.send(Register {
            id: self.player_id.clone(),
            address: ctx.address().clone(),
        });
        request
            .into_actor(self)
            .then(GamePlayer::handle_registration)
            .wait(ctx);
    }
}
impl StreamHandler<actix_web::Result<ws::Message, ws::ProtocolError>> for GamePlayer {
    fn handle(
        &mut self,
        msg: actix_web::Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let to_player_message: serde_json::Result<PlayerMessage> =
                    serde_json::from_str(&text.to_string());
                match to_player_message {
                    Ok(message) => {
                        let _ = ctx.address().try_send(ToPlayer { message });
                        ()
                    }
                    Err(_) => ctx.close(Some(AppError::BadMessage().close_reason())),
                };
            }
            _ => ctx.close(Some(AppError::BadMessage().close_reason())),
        }
    }
}
