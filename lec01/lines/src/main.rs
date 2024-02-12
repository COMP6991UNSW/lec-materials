use bmp::Pixel;

const IMAGE_SIZE_PX: u32 = 256;

fn main() {
    let mut img = bmp::Image::new(
        IMAGE_SIZE_PX, IMAGE_SIZE_PX);

    let red = create_pixel(255, 0, 0);
	let green = create_pixel(0, 255, 0);
	let blue = create_pixel(0, 0, 255);
	let black = create_pixel(0, 0, 0);

    for x in 0..IMAGE_SIZE_PX {
        for y in 0..IMAGE_SIZE_PX {
            if (x + y) % 15 == 0 {
				img.set_pixel(x, y, red);
			} else if (x + y) % 15 == 5 {
				img.set_pixel(x, y, green);
			} else if (x + y) % 15 == 10 {
				img.set_pixel(x, y, blue);
			} else {
				img.set_pixel(x, y, black);
			}

        }
    }

    img.save("my_image.bmp")
        .expect("Failed to save image!");
}

fn create_pixel(r: u8, g: u8, b: u8) -> Pixel {
    let pixel = Pixel::new(r, g, b);

    return pixel;
}