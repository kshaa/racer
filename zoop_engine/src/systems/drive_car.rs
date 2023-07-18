use crate::domain::car_body::CarMeta;

use crate::domain::desync::*;
use crate::domain::game_config::GameConfig;
use crate::domain::ggrs_config::GGRSConfig;
use crate::domain::player::Player;
use crate::domain::tire::{TireMeta, TirePhysics};
use crate::logic::math::*;
use crate::logic::movement::*;

use bevy::prelude::*;
use bevy_ggrs::*;
#[cfg(feature = "debug_lines")]
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

pub fn drive_car(
    config: Res<GameConfig>,
    inputs: Res<PlayerInputs<GGRSConfig>>,
    mut hashes: ResMut<RxFrameHashes>,
    mut source_car_query: Query<(&CarMeta, &Transform, &Player), Without<TireMeta>>,
    mut source_tire_query: Query<
        (
            &mut TirePhysics,
            &TireMeta,
            &mut Transform,
            &Velocity,
            &mut ExternalForce,
            &mut ExternalImpulse,
            &Player,
        ),
        Without<CarMeta>,
    >,
    #[cfg(feature = "debug_lines")] mut lines: ResMut<DebugLines>,
) {
    // Sort queries for more determinism
    let mut car_query = source_car_query.iter_mut().collect::<Vec<_>>();
    car_query.sort_by_key(|(_, _, player)| player.handle);

    let mut tire_query = source_tire_query.iter_mut().collect::<Vec<_>>();
    tire_query.sort_by_key(|(_, meta, _, _, _, _, player)| {
        (
            player.handle,
            if meta.is_front { 1 } else { 0 },
            if meta.is_right { 1 } else { 0 },
        )
    });

    #[cfg(feature = "debug_lines")]
    {
        for (_, car_transform, _) in &car_query {
            let (_, _, car_rotation) = car_transform.rotation.to_euler(EulerRot::XYZ);
            let car_direction = Vec2::from_angle(car_rotation);
            let car_direction3 = Vec3 {
                x: car_direction.x,
                y: car_direction.y,
                z: 0.0,
            } * 60.0;
            lines.line(
                car_transform.translation,
                car_transform.translation + car_direction3,
                0.0,
            );
        }
    }

    for (
        mut tire_physics,
        tire_meta,
        mut transform,
        velocity,
        mut forcable,
        mut impulsable,
        tire_player,
    ) in tire_query
    {
        let (game_input, input_status) = inputs[tire_player.handle];
        if tire_meta.is_front && tire_meta.is_right {
            // Check the desync for this player if they're not a local handle
            // Did they send us some goodies?
            let is_local = config
                .players
                .iter()
                .enumerate()
                .find(|(handle, p)| handle.clone() == tire_player.handle && p.is_local)
                .is_some();
            if !is_local && game_input.last_confirmed_frame > 0 {
                info!("Got frame data {:?}", game_input);
                if let Some(frame_hash) = hashes.0.get_mut(
                    (game_input.last_confirmed_frame as usize) % config.desync_max_frames as usize,
                ) {
                    assert!(
                        frame_hash.frame != game_input.last_confirmed_frame
                            || frame_hash.rapier_checksum == game_input.last_confirmed_hash,
                        "Got new data for existing frame data {}",
                        frame_hash.frame
                    );

                    // Only update this local data if the frame is new-to-us.
                    // We don't want to overwrite any existing validated status
                    // unless the frame is replacing what is already in the buffer.
                    if frame_hash.frame != game_input.last_confirmed_frame {
                        frame_hash.frame = game_input.last_confirmed_frame;
                        frame_hash.rapier_checksum = game_input.last_confirmed_hash;
                        frame_hash.validated = false;
                    }
                }
            }
        }
        if game_input.input > 0 {
            // Useful for desync observing
            debug!(
                "input {:?} from {}: {}",
                input_status, tire_player.handle, game_input.input
            )
        }
        let controls = game_input;

        // Apply controls to tire angle
        tire_physics.angle = tire_physics.angle
            + tire_angle_change(
                tire_meta,
                tire_physics.as_ref(),
                &controls,
                config.tire_max_angle,
                config.tire_rotation_per_tick,
            );

        // Apply tire angle to tire rotation transform
        let (_, car_transform, _) = car_query
            .iter()
            .find(|(_, _, car_player)| car_player.handle == tire_player.handle)
            .unwrap();
        let (_, _, car_rotation) = car_transform.rotation.to_euler(EulerRot::XYZ);
        let tire_rotation = car_rotation + tire_physics.angle;
        let tire_direction = Vec2::from_angle(tire_rotation + deg2rad(90.0)).normalize_or_zero();
        let direction_velocity = velocity.linvel.dot(tire_direction);
        #[cfg(feature = "debug_lines")]
        {
            let tire_direction3 = Vec3 {
                x: tire_direction.x,
                y: tire_direction.y,
                z: 0.0,
            } * 60.0;
            lines.line(
                transform.translation,
                transform.translation + tire_direction3,
                0.0,
            );
        }
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, tire_rotation);

        // Apply tire acceleration
        forcable.force = tire_direction
            * tire_acceleration(
                &tire_meta,
                &controls,
                config.pixels_per_meter,
                direction_velocity,
                config.tire_acceleration_force,
                config.tire_reversing_force,
                config.tire_breaking_force,
            );

        // Apply friction
        let friction_impulse = tire_friction_impulse(
            config.tire_friction_force,
            &tire_direction,
            &velocity.linvel,
        );
        #[cfg(feature = "debug_lines")]
        {
            let friction_impulse3 = Vec3 {
                x: friction_impulse.x,
                y: friction_impulse.y,
                z: 0.0,
            } * 60.0;
            lines.line(
                transform.translation,
                transform.translation + friction_impulse3,
                0.0,
            );
        }
        impulsable.impulse += friction_impulse;
    }
}
