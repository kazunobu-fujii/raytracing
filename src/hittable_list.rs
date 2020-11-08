use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList<T> {
    objects: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new(object: T) -> HittableList<T> {
        HittableList {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
