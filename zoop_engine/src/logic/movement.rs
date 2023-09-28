use crate::domain::controls::Controls;
use crate::domain::tire::*;
use crate::logic::math::*;
use bevy::math::*;
use nalgebra::ComplexField;

pub fn tire_angle_change(
    tire_meta: &TireMeta,
    tire_physics: &TirePhysics,
    controls: &Controls,
    min_steering_angle: f32,
    max_steering_angle: f32,
    rotation_step: f32,
    tire_rotation_velocity_reduction_coefficient: f32,
    tire_rotation_step_reduction_coefficient: f32,
    tire_velocity: f32,
) -> f32 {
    let velocity_reduced_max_angle = min_steering_angle
        .max(max_steering_angle - tire_velocity * tire_rotation_velocity_reduction_coefficient);

    let velocity_reduced_rotation_step =
        (rotation_step * tire_velocity.abs() * tire_rotation_step_reduction_coefficient)
            .min(rotation_step);

    if !tire_meta.is_front {
        // Back wheels don't steer
        0.0
    } else {
        if controls.steering_left() && tire_physics.angle < velocity_reduced_max_angle {
            // Steering left
            velocity_reduced_rotation_step
        } else if controls.steering_right() && tire_physics.angle > -velocity_reduced_max_angle {
            // Steering right
            -velocity_reduced_rotation_step
        } else if !controls.steering_any() {
            // Steering back to center
            if tire_physics.angle < velocity_reduced_rotation_step
                && tire_physics.angle > -velocity_reduced_rotation_step
            {
                // Already at center
                0.0
            } else if tire_physics.angle <= -velocity_reduced_rotation_step {
                // Steering from left
                velocity_reduced_rotation_step
            } else {
                // Steering from right
                -velocity_reduced_rotation_step
            }
        } else {
            // Steering over treshold - keeping same angle
            0.0
        }
    }
}

pub fn tire_acceleration(
    tire_meta: &TireMeta,
    controls: &Controls,
    pixels_per_meter: f32,
    direction_velocity: f32,
    acceleration_force: f32,
    reversing_force: f32,
    breaking_force: f32,
    parking_multiplier: f32,
) -> f32 {
    let controlled_parking_multiplier = if controls.parking() {
        parking_multiplier
    } else {
        1.0
    };
    if !tire_meta.is_front {
        // Only front wheels accelerate currently
        0.0
    } else {
        if controls.breaking() && direction_velocity.abs() > pixels_per_meter * 0.05 {
            // Breaking deceleration
            signed(direction_velocity < 0.0, breaking_force)
        } else if controls.reversing() {
            // Backwards acceleration
            -reversing_force * controlled_parking_multiplier
        } else if controls.accelerating() {
            // Forwards acceleration
            acceleration_force * controlled_parking_multiplier
        } else {
            // No acceleration
            0.0
        }
    }
}

pub fn tire_friction_impulse(
    is_drifting: bool,
    drift_leftover: f32,
    tire_friction_force: f32,
    tire_direction: &Vec2,
    tire_velocity: &Vec2,
) -> Vec2 {
    let velocity_angle_unsafe = tire_direction.angle_between(tire_velocity.to_owned());
    let velocity_angle = if velocity_angle_unsafe.is_nan() {
        0.0
    } else {
        velocity_angle_unsafe
    };
    let friction_force = if is_drifting {
        0.0
    } else if drift_leftover != 0.0 {
        tire_friction_force * (1.0 - drift_leftover)
    } else {
        tire_friction_force
    };
    let slide_amount =
        ComplexField::sin(velocity_angle).abs() * tire_velocity.length() * friction_force;
    let slide_direction = if velocity_angle < 0.0 {
        deg2rad(90.0)
    } else {
        deg2rad(-90.0)
    };
    let friction_impulse =
        Vec2::from_angle(slide_direction).rotate(tire_direction.normalize_or_zero()) * slide_amount;

    friction_impulse
}
