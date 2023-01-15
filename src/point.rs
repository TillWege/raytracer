use crate::vec3::Vec3;

pub(crate) type Point = Vec3;

impl Point{
    pub(crate) fn distance_from_origin(&self) -> f32 {
        self.length()
    }


    pub(crate) fn distance(&self, other: &Self) -> f32 {
        (self - other).length()
    }
}