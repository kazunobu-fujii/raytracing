use raytracing::camera::Camera;
use raytracing::color::write_color;
use raytracing::hittable::{HitRecord, Hittable};
use raytracing::hittable_list::HittableList;
use raytracing::ray::Ray;
use raytracing::rtweekend::{random_double, INFINITY};
use raytracing::sphere::Sphere;
use raytracing::vec3::{Color, Point3, Vec3};

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
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
                let u = (i as f32 + random_double()) / (image_width - 1) as f32;
                let v = (j as f32 + random_double()) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprint!("\nDone.\n");
}
