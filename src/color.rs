use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(pixel_color: Color) {
        let r = (255.999 * pixel_color.x()) as u8;
        let g = (255.999 * pixel_color.y()) as u8;
        let b = (255.999 * pixel_color.z()) as u8;

        print!("{} {} {}\n", r, g, b);
    }
}
