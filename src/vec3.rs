use rand::distributions::{Distribution, Uniform};
use std::ops;
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        (&self.x() * v.x()) + (&self.y() * v.y()) + (&self.z() * v.z())
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * v.z() - self.z() * v.y(),
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
        )
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    pub fn random() -> Self {
        let (x, y, z) = rand::random();
        Self::new(x, y, z)
    }
    pub fn random_in_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let range = Uniform::from(min..max);
        Self::new(
            range.sample(&mut rng),
            range.sample(&mut rng),
            range.sample(&mut rng),
        )
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn to_unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random();
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random().to_unit_vector()
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let on_unit_sphere = Self::random_in_range(-1., 1.).to_unit_vector();
        if on_unit_sphere.dot(&normal) < 0. {
            return -on_unit_sphere;
        } else {
            return on_unit_sphere;
        };
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(mut self) -> Vec3 {
        self.e[0] = -self.e[0];
        self.e[1] = -self.e[1];
        self.e[2] = -self.e[2];
        self
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self.e[0] += v.e[0];
        self.e[1] += v.e[1];
        self.e[2] += v.e[2];
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        self.e[0] -= v.e[0];
        self.e[1] -= v.e[1];
        self.e[2] -= v.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.e[0] /= t;
        self.e[1] /= t;
        self.e[2] /= t;
    }
}

impl ops::Index<i32> for Vec3 {
    type Output = f64;
    fn index(&self, index: i32) -> &f64 {
        // TODO: get rid of this type casting
        &self.e[index as usize]
    }
}

pub use Vec3 as Point3;

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Self::Output {
        Vec3::new(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Self::Output {
        Vec3::new(self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2])
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;
    fn add(self, v: &Vec3) -> Self::Output {
        Vec3::new(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, v: &Vec3) -> Self::Output {
        Vec3::new(self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2])
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2])
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Self::Output {
        Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Self::Output {
        Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        v * self
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: &Vec3) -> Self::Output {
        v * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Self::Output {
        (1. / t) * self
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Self::Output {
        (1. / t) * self
    }
}
