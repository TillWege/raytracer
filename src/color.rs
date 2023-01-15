use crate::vec3::Vec3;

pub(crate) type Color = Vec3;

impl Color {
    pub(crate) fn Black() -> Self{
        Self::new(0.0, 0.0, 0.0)
    }

    pub(crate) fn White() -> Self{
        Self::new(1.0, 1.0, 1.0)
    }

    pub(crate) fn to_rgb(&self) -> image::Rgb<u8> {
        image::Rgb([(self.x() * 256.0) as u8, (self.y() * 256.0) as u8, (self.z() * 256.0) as u8])
    }
}