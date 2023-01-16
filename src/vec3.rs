use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    pub(crate) fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub(crate) fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub(crate) fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub(crate) fn cross(&self, rhs: Self) -> Self {
        Self { 
            x: self.y * rhs.z - self.z * rhs.y, 
            y: self.z * rhs.x - self.x * rhs.z, 
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub(crate) fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub(crate) fn x(&self) -> f32 {
        self.x
    }

    pub(crate) fn y(&self) -> f32 {
        self.y
    }

    pub(crate) fn z(&self) -> f32 {
        self.z
    }

}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::zero()
    }
}

//
// Operator Overloads for Adding
//
impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        Vec3 { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + &rhs
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        &self + rhs
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        self + &rhs
    }
}

//
// Operator Overloads for Subtracting
//

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 { 
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - &rhs
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self {
        &self - rhs
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        self - &rhs
    }
}

//
// Operator Overloads for Multiplying
//
impl Mul<&f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &f32) -> Vec3 {
        Vec3 { 
            x: self.x * rhs, 
            y: self.y * rhs, 
            z: self.z * rhs
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        self * &rhs
    }
}


impl Mul<&f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: &f32) -> Self {
        &self * rhs
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        self * &rhs
    }
}

//
// Operator Overloads for Multipling
//

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * &self
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        rhs * &self
    }
}

impl Mul<Vec3> for &f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Mul<&Vec3> for &f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        rhs * self
    }
}


//
// Operator Overloads for Dividing
//
impl Div<&f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &f32) -> Vec3 {
        self * (1.0 / rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        &self / &rhs
    }
}

impl Div<&f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: &f32) -> Self {
        &self / rhs
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        self / &rhs
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
    fn new() {
        let v = super::Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
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

    #[test]
    fn dot() {
        let v1 = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = super::Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn cross() {
        let v1 = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = super::Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(v1.cross(v2), super::Vec3 { x: -3.0, y: 6.0, z: -3.0 });
    }

    #[test]
    fn unit_vector() {
        let v1 = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = super::Vec3 { x: 0.26726124, y: 0.5345225, z: 0.8017837 };
        assert_eq!(v1.unit_vector(), v2);
    }

    #[test]
    fn unit_vector_length() {
        let v1 = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(v1.unit_vector().length().round(), (1.0 as f32));
        
    }

    #[test]
    fn test_add_impl() {
        let v1 = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = super::Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let v3 = super::Vec3 { x: 5.0, y: 7.0, z: 9.0 };

        assert_eq!(v1 + &v2, v3);
        assert_eq!(&v1 + v2, v3);
        assert_eq!(&v1 + &v2, v3);
        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn test_sub_impl() {
        let v1 = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = super::Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let v3 = super::Vec3 { x: 5.0, y: 7.0, z: 9.0 };

        assert_eq!(v3 - &v2, v1);
        assert_eq!(&v3 - v2, v1);
        assert_eq!(&v3 - &v2, v1);
        assert_eq!(v3 - v2, v1);
    }

    #[test]
    fn test_mul_impl() {
        let v1 = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = super::Vec3 { x: 2.0, y: 4.0, z: 6.0 };
        assert_eq!(v1 * &2.0, v2);
        assert_eq!(&v1 * 2.0, v2);
        assert_eq!(&v1 * &2.0, v2);
        assert_eq!(v1 * 2.0, v2);
    }

    #[test]
    fn test_div_impl() {
        let v1 = super::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = super::Vec3 { x: 0.5, y: 1.0, z: 1.5 };
        assert_eq!(v1 / &2.0, v2);
        assert_eq!(&v1 / 2.0, v2);
        assert_eq!(&v1 / &2.0, v2);
        assert_eq!(v1 / 2.0, v2);
    }


}