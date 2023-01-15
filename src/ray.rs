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
    
