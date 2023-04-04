use nalgebra::RealField;

pub fn deg2rad(degrees: f32) -> f32 {
    let pi: f32 = RealField::pi();
    let radian = pi / 180.0;
    degrees * radian
}

pub fn signed(s: bool, a: f32) -> f32 {
    if s {
        a
    } else {
        -a
    }
}
