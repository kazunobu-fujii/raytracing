#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e: [f32; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new_zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.e[idx]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.e[idx]
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        };
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        };
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            e: [self.e[0] / other, self.e[1] / other, self.e[2] / other],
        };
    }
}

impl Vec3 {
    pub fn length_squared(self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl Vec3 {
    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }
    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_zero() {
        let v = Vec3::new_zero();
        assert_eq!(v.e[0], 0.0);
        assert_eq!(v.e[1], 0.0);
        assert_eq!(v.e[2], 0.0);
    }
    #[test]
    fn test_new() {
        let v = Vec3::new(1.1, 2.2, 3.3);
        assert_eq!(v.e[0], 1.1);
        assert_eq!(v.e[1], 2.2);
        assert_eq!(v.e[2], 3.3);
        assert_eq!(v.x(), 1.1);
        assert_eq!(v.y(), 2.2);
        assert_eq!(v.z(), 3.3);
    }
    #[test]
    fn test_neg() {
        let v = -Vec3::new(1.1, 2.2, 3.3);
        assert_eq!(v.e[0], -1.1);
        assert_eq!(v.e[1], -2.2);
        assert_eq!(v.e[2], -3.3);
    }
    #[test]
    fn test_index() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }
    #[test]
    fn test_index_mut() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v[0] += 0.1;
        v[1] += 0.2;
        v[2] += 0.3;
        assert_eq!(v[0], 1.1);
        assert_eq!(v[1], 2.2);
        assert_eq!(v[2], 3.3);
    }
    #[test]
    fn test_add() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let a = Vec3::new(0.1, 0.2, 0.3);
        v += a;
        assert_eq!(v.e[0], 1.1);
        assert_eq!(v.e[1], 2.2);
        assert_eq!(v.e[2], 3.3);
        let b = v + a;
        assert_eq!(b.e[0], 1.2);
        assert_eq!(b.e[1], 2.4);
        assert_eq!(b.e[2], 3.6);
        let c = v - a;
        assert_eq!(c.e[0], 1.0);
        assert_eq!(c.e[1], 2.0);
        assert_eq!(c.e[2], 3.0);
    }
    #[test]
    fn test_mul() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v.e[0], 2.0);
        assert_eq!(v.e[1], 4.0);
        assert_eq!(v.e[2], 6.0);
        let a = Vec3::new(0.1, 0.2, 0.4);
        let b = v * a;
        assert_eq!(b.e[0], 0.2);
        assert_eq!(b.e[1], 0.8);
        assert_eq!(b.e[2], 2.4);
        let c = v * 0.1;
        assert_eq!(c.e[0], 0.2);
        assert_eq!(c.e[1], 0.4);
        assert_eq!(c.e[2], 0.6);
        let d = 0.1 * v;
        assert_eq!(d.e[0], 0.2);
        assert_eq!(d.e[1], 0.4);
        assert_eq!(d.e[2], 0.6);
    }
    #[test]
    fn test_div() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v /= 2.0;
        assert_eq!(v.e[0], 0.5);
        assert_eq!(v.e[1], 1.0);
        assert_eq!(v.e[2], 1.5);
        let a = v / 0.5;
        assert_eq!(a.e[0], 1.0);
        assert_eq!(a.e[1], 2.0);
        assert_eq!(a.e[2], 3.0);
    }
    #[test]
    fn test_sqrt() {
        let v = Vec3::new(1.0, 4.0, 8.0);
        assert_eq!(v.length_squared(), 81.0);
        assert_eq!(v.length(), 9.0);
    }
    #[test]
    fn test_display() {
        let v = Vec3::new(1.1, 2.2, 3.3);
        assert_eq!(format!("{}", v), "1.1 2.2 3.3")
    }
    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let x = Vec3::dot(&v1, &v2);
        assert_eq!(x, 32.0);
    }
    #[test]
    fn test_corss() {
        let v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(-1.0, 0.0, 3.0);
        let x = Vec3::cross(&v1, &v2);
        assert_eq!(x.e[0], 3.0);
        assert_eq!(x.e[1], -2.0);
        assert_eq!(x.e[2], 1.0);
    }
    #[test]
    fn test_unit_vector() {
        let v = Vec3::new(10.0, 0.0, 0.0);
        let x = Vec3::unit_vector(v);
        assert_eq!(x.e[0], 1.0);
        assert_eq!(x.e[1], 0.0);
        assert_eq!(x.e[2], 0.0);
    }
}
