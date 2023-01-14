mod vec3;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    let mut image = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);


    for y in 0..IMAGE_HEIGHT {
        println!("Rendering Row {}", y);
        for x in 0..IMAGE_WIDTH {
            let pixel = image.get_pixel_mut(x, y);
            
            let r = x as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = y as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;

            *pixel = image::Rgb([(r * 255.99) as u8, (g * 255.99) as u8, b as u8]);
        }
    }
    

    if let Err(e) = image.save("image.png") {
        println!("Error saving image: {}", e);
    }
}
