#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::float_arithmetic,
    clippy::print_stdout,
    clippy::print_stderr
)]
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use rtweekend::{random_double, INFINITY};
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new_zero();
    }
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target = rec.p + Vec3::random_in_hemisphere(&rec.normal);
        return 0.5
            * ray_color(
                &Ray::new(rec.p, target - rec.p),
                world,
                depth.saturating_sub(1),
            );
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i16 = 400;
    #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
    let image_height: i16 = (f32::from(image_width) / aspect_ratio) as i16;
    let samples_per_pixel = 100;
    let max_depth = 50;
    // World
    let mut world = HittableList::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });
    // Camera
    let cam = Camera::new();
    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (f32::from(i) + random_double()) / f32::from(image_width - 1);
                let v = (f32::from(j) + random_double()) / f32::from(image_height - 1);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            color::write(pixel_color, samples_per_pixel);
        }
    }
    eprint!("\nDone.\n");
}
