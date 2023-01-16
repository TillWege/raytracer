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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(4.0, 5.0, 6.0);
        assert_eq!(point1.distance(&point2), 5.196152);
    }

    #[test]
    fn test_distance_from_origin() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(point.distance_from_origin(), 3.7416575);
    }
}