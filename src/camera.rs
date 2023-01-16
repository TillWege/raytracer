use crate::vec3::{Vec3};

const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub(crate) const IMAGE_WIDTH: u32 = 800;
pub(crate) const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

pub(crate) struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub(crate) fn bottom_left(&self) -> Vec3 {
        self.origin - (self.horizontal / 2.0) - (self.vertical / 2.0) - Vec3::new(0.0, 0.0, FOCAL_LENGTH)
    }

    pub(crate) fn horizontal(&self) -> Vec3 {
        self.horizontal
    }

    pub(crate) fn vertical(&self) -> Vec3 {
        self.vertical
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Default::default(),
            horizontal: Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0),
            vertical: Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0),
        }
    }
}
