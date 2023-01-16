use crate::{ray::Ray, vec3::Vec3};

pub(crate) struct Sphere {
    origin: Vec3,
    radius: f32,
}

impl Sphere {
    pub(crate) fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin() - self.origin;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(&oc) - (self.radius * self.radius);

        let discriminant = b*b - 4.0*a*c;
        
        return discriminant > 0.0;
    }

    pub(crate) fn new(origin: Vec3, radius: f32) -> Sphere {
        Self { origin, radius }
    }
}
