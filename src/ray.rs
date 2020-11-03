use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
}
impl Ray {
    pub fn origin(self) -> Point3 {
        self.orig
    }
    pub fn direction(self) -> Vec3 {
        self.dir
    }
    pub fn at(self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}