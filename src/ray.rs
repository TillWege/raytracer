use crate::vec3::{Vec3};
use crate::point::{Point};

pub(crate) struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub(crate) fn origin(&self) -> &Point {
        &self.origin
    }

    pub(crate) fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub(crate) fn at(&self, t: f32) -> Point {
        self.origin + (self.direction * t)
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::new(Point::zero(), Vec3::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(ray.origin().x(), 1.0);
        assert_eq!(ray.origin().y(), 2.0);
        assert_eq!(ray.origin().z(), 3.0);
        assert_eq!(ray.direction().x(), 4.0);
        assert_eq!(ray.direction().y(), 5.0);
        assert_eq!(ray.direction().z(), 6.0);
    }

    #[test]
    fn test_at() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        let point = ray.at(2.0);
        assert_eq!(point.x(), 9.0);
        assert_eq!(point.y(), 12.0);
        assert_eq!(point.z(), 15.0);
    }
}