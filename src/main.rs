use camera::{Camera, IMAGE_WIDTH, IMAGE_HEIGHT};
use color::Color;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

mod vec3;
mod color;
mod point;
mod ray;
mod camera;
mod sphere;

fn main() {

    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let camera = Camera::default();
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);



    for y in 0..IMAGE_HEIGHT {
        println!("Rendering Row {}", y);
        for x in 0..IMAGE_WIDTH {
            let pixel = image.get_pixel_mut(x, IMAGE_HEIGHT-1  - y);
            
            let u = x as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = y as f32 / (IMAGE_HEIGHT - 1) as f32;

            let ray: Ray = Ray::new(Vec3::zero(), camera.bottom_left() + (u * camera.horizontal()) + (v * camera.vertical()));
            if sphere.hit(&ray) {
                *pixel = Color::Red().to_rgb();
            } else {
                *pixel = ray_color(&ray).to_rgb();
            }


        }
    }

    if let Err(e) = image.save("image.png") {
        println!("Error saving image: {}", e);
    }
}

fn ray_color(ray: &Ray) -> Color {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Color::White() + t * Color::new(0.5, 0.7, 1.0);
}
