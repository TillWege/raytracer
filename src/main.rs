mod vec3;
mod color;
mod point;
mod ray;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {

    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in 0..IMAGE_HEIGHT {
        println!("Rendering Row {}", y);
        for x in 0..IMAGE_WIDTH {
            let pixel = image.get_pixel_mut(x, y);
            
            let color = color::Color::new(x as f32 / (IMAGE_WIDTH - 1) as f32, y as f32 / (IMAGE_HEIGHT - 1) as f32, 0.25);

            *pixel = color.to_rgb();
        }
    }
    


    if let Err(e) = image.save("image.png") {
        println!("Error saving image: {}", e);
    }
}
