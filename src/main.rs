const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    let image = image::ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let r = x as f32 / (IMAGE_WIDTH - 1) as f32;
        let g = y as f32 / (IMAGE_HEIGHT - 1) as f32;
        let b = 0.25;

        image::Rgb([(r * 255.99) as u8, (g * 255.99) as u8, (b * 255.99) as u8])
    });

    if let Err(e) = image.save("image.png") {
        println!("Error saving image: {}", e);
    }
}
