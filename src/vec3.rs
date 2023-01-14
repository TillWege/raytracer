use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(&self, rhs: Self) -> Self {
        Self { 
            x: self.y * rhs.z - self.z * rhs.y, 
            y: self.z * rhs.x - self.x * rhs.z, 
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    fn unit_vector(&self) -> Self {
        *self / self.length()
    }


}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::zero()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { 
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self { 
            x: self.x * rhs, 
            y: self.y * rhs, 
            z: self.z * rhs
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        self * (1.0 / rhs)
    }
}

#[cfg(test)]
mod tests {


    #[test]
    fn creation() {
        let v = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.length_squared(), 14.0);
        assert_eq!(v.length(), 3.7416575);

        let v0 = super::Vec3::zero();
        assert_eq!(v0.x, 0.0);
        assert_eq!(v0.y, 0.0);
        assert_eq!(v0.z, 0.0);
        assert_eq!(v0.length_squared(), 0.0);
        assert_eq!(v0.length(), 0.0);

        let v1 = super::Vec3::default();
        assert_eq!(v1, v0);
    }

    #[test]
    fn math() {
        let v1 = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = super::Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let v3 = super::Vec3 { x: 5.0, y: 7.0, z: 9.0 };

        assert_eq!(v1 + v2, v3);
        assert_eq!(v3 - v2, v1);
        assert_eq!(v1 * 2.0, super::Vec3 { x: 2.0, y: 4.0, z: 6.0 });
        assert_eq!(v1 / 2.0, super::Vec3 { x: 0.5, y: 1.0, z: 1.5 });

        // mutltiplaction test of a vector and a f32
        let v4 = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v5 = super::Vec3 { x: 4.0, y: 8.0, z: 12.0 };
        assert_eq!(v4 * 4.0, v5);

        // division test of a vector and a f32
        let v6 = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v7 = super::Vec3 { x: 0.5, y: 1.0, z: 1.5 };
        assert_eq!(v6 / 2.0, v7);
    }

}