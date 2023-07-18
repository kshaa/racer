use crate::domain::frames::*;
use crate::domain::game_config::DESYNC_MAX_FRAMES;
use crate::domain::ggrs_config::GGRSConfig;
use bevy::prelude::*;
use bevy_ggrs::{Rollback, Session};
use bevy_rapier2d::prelude::*;
use ggrs::*;

/// Metadata we need to store about frames we've rendered locally
#[derive(Default, Hash, Resource, PartialEq, Eq, Debug)]
pub struct FrameHash {
    /// The frame number for this metadata
    pub frame: Frame,

    /// The checksum of the Rapier physics state for the frame.  I use this term interchangably with `hash`, sorry.
    pub rapier_checksum: u16,

    /// Has been confirmed by GGRS
    pub confirmed: bool,

    /// Has been sent by us to other players
    pub sent: bool,

    /// Has been validated by us against other player
    pub validated: bool,
}

/// Metadata we need to store about frames we've received from other player
#[derive(Default, Hash, Resource, PartialEq, Eq, Debug)]
pub struct RxFrameHash {
    /// The frame number for this metadata
    pub frame: Frame,

    /// The checksum of the Rapier physics state for the frame.  I use this term interchangably with `hash`, sorry.
    pub rapier_checksum: u16,

    /// Has been validated by us against other player
    pub validated: bool,
}

// A collection of confirmed frame hashes we've seen locally
#[derive(Default, Hash, Resource, PartialEq, Eq)]
pub struct FrameHashes(pub [FrameHash; DESYNC_MAX_FRAMES as usize]);

// A collection of confirmed frame hashes we've received from our other player
// This only works for 1v1.  This would have to be extended to consider all
// remotes in larger scenarios (I accept pull requests!)
#[derive(Default, Hash, Resource, PartialEq, Eq)]
pub struct RxFrameHashes(pub [RxFrameHash; DESYNC_MAX_FRAMES as usize]);

/// Our desync detector!
/// Validates the hashes we've received so far against the ones we've calculated ourselves.
/// If there is a difference, panic.  Your game will probably want to handle this more gracefully.
pub fn frame_validator(
    mut hashes: ResMut<FrameHashes>,
    mut rx_hashes: ResMut<RxFrameHashes>,
    validatable_frame: Res<ValidatableFrame>,
    _session: ResMut<Session<GGRSConfig>>,
) {
    #[cfg(feature = "ggrs_desync_detection")]
    {
        if let Session::P2PSession(s) = session.as_mut() {
            let events = s.events().collect::<Vec<GGRSEvent<GGRSConfig>>>();
            for event in events {
                if let GGRSEvent::DesyncDetected {
                    /// Frame of the checksums
                    frame,
                    /// local checksum for the given frame
                    local_checksum,
                    /// remote checksum for the given frame
                    remote_checksum,
                    /// remote address of the endpoint.
                    addr,
                } = event
                {
                    let msg = format!(
                        "Desync on frame {:?}, local checksum {:?} != remote checksum {:?} for address {:?}",
                        frame,
                        local_checksum,
                        remote_checksum,
                        addr);
                    panic!("{}", msg);
                }
            }
        }
    }
    for (i, rx) in rx_hashes.0.iter_mut().enumerate() {
        // Check every confirmed frame that has not been validated
        if rx.frame > 0 && !rx.validated {
            // Get that same frame in our buffer
            if let Some(sx) = hashes.0.get_mut(i) {
                // Make sure it's the exact same frame and also confirmed and not yet validated
                // and importantly is SAFE to validate
                if sx.frame == rx.frame
                    && sx.confirmed
                    && !sx.validated
                    && validatable_frame.is_validatable(sx.frame)
                {
                    // If this is causing your game to exit, you have a bug!
                    let checksums_match = sx.rapier_checksum == rx.rapier_checksum;
                    if !checksums_match {
                        error!("Failed checksum checks {:?} != {:?}", sx, rx);
                        panic!("Failed checksum checks {:?} != {:?}", sx, rx);
                    }

                    // Set both as validated
                    info!("Frame validated {:?}", sx.frame);
                    sx.validated = true;
                    rx.validated = true;
                }
            }
        }
    }
}

pub fn force_update_rollbackables(
    mut t_query: Query<&mut Transform, With<Rollback>>,
    mut v_query: Query<&mut Velocity, With<Rollback>>,
) {
    for mut t in t_query.iter_mut() {
        t.set_changed();
    }
    for mut v in v_query.iter_mut() {
        v.set_changed();
    }
}
