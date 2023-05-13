use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierContext;
use crate::domain::checksum::fletcher16;
use crate::domain::game_config::GameConfig;
use crate::domain::rapier_rollback_state::RapierRollbackState;

pub fn save_rapier_context(
    config: Res<GameConfig>,
    mut game_state: ResMut<RapierRollbackState>,
    rapier: Res<RapierContext>,
    mut hashes: ResMut<FrameHashes>,
    confirmed_frame: Res<ConfirmedFrame>,
    current_frame: Res<CurrentFrame>,
) {
    // This serializes our context every frame.  It's not great, but works to
    // integrate the two plugins.  To do less of it, we would need to change
    // bevy_ggrs to serialize arbitrary structs like this one in addition to
    // component tracking.  If you need this to happen less, I'd recommend not
    // using the plugin and implementing GGRS yourself.
    if let Ok(context_bytes) = bincode::serialize(rapier.as_ref()) {
        info!("Context hash before save: {}", game_state.rapier_checksum);
        game_state.rapier_checksum = fletcher16(&context_bytes);
        game_state.rapier_state = Some(context_bytes);
        info!("Context hash after save: {}", game_state.rapier_checksum);

        if let Some(frame_hash) = hashes
            .0
            .get_mut((current_frame.0 as usize) % config.desync_max_frames)
        {
            if frame_hash.frame == current_frame.0 && frame_hash.sent {
                // If this frame hash has already been sent and its the
                // same one then the hashes better damn well match
                assert_eq!(
                    frame_hash.rapier_checksum, game_state.rapier_checksum,
                    "INTEGRITY BREACHED"
                );
                info!(
                    "Integrity challenged of frame {}: {} vs {}",
                    frame_hash.frame,
                    frame_hash.rapier_checksum,
                    game_state.rapier_checksum
                );
            }

            frame_hash.frame = current_frame.0;
            frame_hash.rapier_checksum = game_state.rapier_checksum;
            frame_hash.sent = false;
            frame_hash.validated = false;
            debug!("confirmed frame: {:?}", confirmed_frame);
            frame_hash.confirmed = frame_hash.frame <= confirmed_frame.0;
            debug!("Stored frame hash at save: {:?}", frame_hash);
        }

        info!("----- end frame {} -----", current_frame.0);
    }
}