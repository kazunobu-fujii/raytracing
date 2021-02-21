use crate::rtweekend::clamp;
use crate::vec3::Color;

pub fn write(pixel_color: Color, samples_per_pixel: i16) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / f32::from(samples_per_pixel);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::as_conversions
    )]
    let to_byte = |v: f32| -> u8 { (256.0 * clamp(v, 0.0, 0.999)) as u8 };
    println!("{} {} {}", to_byte(r), to_byte(g), to_byte(b),);
}
