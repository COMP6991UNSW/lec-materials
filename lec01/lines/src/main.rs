use bmp::{Image, Pixel};

const IMAGE_SIZE_PX: u32 = 256;

fn main() {
    let mut image = Image::new(IMAGE_SIZE_PX, IMAGE_SIZE_PX);
    
    let red = create_pixel(255, 0, 0);
    let green = create_pixel(0, 255, 0);
    let blue = create_pixel(0, 0, 255);
    let black = create_pixel(0, 0, 0);

    for x in 0..IMAGE_SIZE_PX {
        for y in 0..IMAGE_SIZE_PX {
            let mod15 = (x + y) % 15;
            let color = if mod15 == 0 {
                red
            } else if mod15 == 5 {
                green
            } else if mod15 == 10 {
                blue
            } else {
                black
            };

            image.set_pixel(x, y, color);
        }
    }

    image.save("my_image.bmp")
        .expect("Failed to save image!");
}

fn create_pixel(r: u8, g: u8, b: u8) -> Pixel {
    Pixel::new(r, g, b)
}
