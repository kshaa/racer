use crate::controls::*;
use crate::math::*;
use crate::tire::*;
use bevy::math::*;
use nalgebra::ComplexField;

pub fn tire_angle_change(
    tire_meta: &TireMeta,
    controls: &Controls,
    max_steering_angle: f32,
    rotation_step: f32,
) -> f32 {
    if !tire_meta.is_front {
        // Back wheels don't steer
        0.0
    } else {
        if controls.steering_left() && tire_meta.angle < max_steering_angle {
            // Steering left
            rotation_step
        } else if controls.steering_right() && tire_meta.angle > -max_steering_angle {
            // Steering right
            -rotation_step
        } else if !controls.steering_any() {
            // Steering back to center
            if tire_meta.angle < rotation_step && tire_meta.angle > -rotation_step {
                // Already at center
                0.0
            } else if tire_meta.angle <= -rotation_step {
                // Steering from left
                rotation_step
            } else {
                // Steering from right
                -rotation_step
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
) -> f32 {
    if !tire_meta.is_front {
        // Only front wheels accelerate currently
        0.0
    } else {
        if controls.breaking() && direction_velocity.abs() > pixels_per_meter * 0.05 {
            // Breaking deceleration
            signed(direction_velocity < 0.0, breaking_force)
        } else if controls.reversing() {
            // Backwards acceleration
            -reversing_force
        } else if controls.accelerating() {
            // Forwards acceleration
            acceleration_force
        } else {
            // No acceleration
            0.0
        }
    }
}

pub fn tire_friction_impulse(
    tire_friction_pushback_percentage: f32,
    tire_direction: &Vec2,
    tire_velocity: &Vec2,
) -> Vec2 {
    let velocity_angle_unsafe = tire_direction.angle_between(tire_velocity.to_owned());
    let velocity_angle = if velocity_angle_unsafe.is_nan() {
        0.0
    } else {
        velocity_angle_unsafe
    };
    let slide_amount = ComplexField::sin(velocity_angle).abs()
        * tire_velocity.length()
        * tire_friction_pushback_percentage;
    let slide_direction = if velocity_angle < 0.0 {
        deg2rad(90.0)
    } else {
        deg2rad(-90.0)
    };
    let friction_impulse =
        Vec2::from_angle(slide_direction).rotate(tire_direction.normalize_or_zero()) * slide_amount;

    friction_impulse
}
