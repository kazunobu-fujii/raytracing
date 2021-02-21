use rand::prelude::*;

pub const INFINITY: f32 = std::f32::INFINITY;

pub fn random_double() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_double_with(min: f32, max: f32) -> f32 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
