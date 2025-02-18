use bmp::{Image, Pixel};

const IMAGE_SIZE_PX: u32 = 256;

fn main() {
    let mut my_image = Image::new(IMAGE_SIZE_PX, IMAGE_SIZE_PX);

    let red = create_pixel(255, 0, 0);
    let green = create_pixel(0, 255, 0);
    let blue = create_pixel(0, 0, 255);
    let black = create_pixel(0, 0, 0);

    for x in 0..IMAGE_SIZE_PX {
        for y in 0..IMAGE_SIZE_PX {
            let colour = if (x + y) % 15 == 0 {
                red
            } else if (x + y) % 15 == 5 {
                green
            } else if (x + y) % 15 == 10 {
                blue
            } else {
                black
            };

            // let colour = match (x + y) % 15 {
            //     0 => red,
            //     5 => green,
            //     10 => blue,
            //     _ => black,
            // };

            my_image.set_pixel(x, y, colour);
        }
    }

    let result = my_image.save("my_image.bmp");
    result.expect("Failed to save image");
}

fn create_pixel(r: u8, g: u8, b: u8) -> Pixel {
    let pixel = Pixel::new(r, g, b);
    return pixel;
}
