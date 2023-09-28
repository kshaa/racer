use nalgebra::RealField;

pub fn deg2rad(degrees: f32) -> f32 {
    let pi: f32 = RealField::pi();
    let conversion = pi / 180.0;
    degrees * conversion
}

pub fn rad2deg(degrees: f32) -> f32 {
    let pi: f32 = RealField::pi();
    let conversion = 180.0 / pi;
    degrees * conversion
}

pub fn signed(s: bool, a: f32) -> f32 {
    if s {
        a
    } else {
        -a
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::*;
    use bevy::prelude::{Transform, Vec3};
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_deg2rad() {
        assert_f32_near!(deg2rad(90.0), 1.5707964);
    }

    #[test]
    fn test_vector_rotate_by_deg() {
        let p1 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let t1 = Transform::from_translation(p1);
        let mut t2 = t1.clone();
        t2.rotate_local_z(deg2rad(90.0));
        let r = t1.rotation.angle_between(t2.rotation);
        assert_f32_near!(r, deg2rad(90.0));
    }
}
