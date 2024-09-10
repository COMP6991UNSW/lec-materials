use bmp::Image;
use bmp::Pixel;

const IMAGE_SIZE_PX: u32 = 256;

fn main() {
	let mut image = Image::new(IMAGE_SIZE_PX, IMAGE_SIZE_PX);
	
	let red = create_pixel(255, 0, 0);
	let green = create_pixel(0, 255, 0);
	let blue = create_pixel(0, 0, 255);
	let black = create_pixel(0, 0, 0);

	for x in 0..IMAGE_SIZE_PX {
		for y in 0..IMAGE_SIZE_PX {
			if (x + y) % 15 == 0 {
				image.set_pixel(x, y, red);
			} else if (x + y) % 15 == 5 {
				image.set_pixel(x, y, green);
			} else if (x + y) % 15 == 10 {
				image.set_pixel(x, y, blue);
			} else {
				image.set_pixel(x, y, black);
			}
		}
	}

	image.save("my_image.bmp")
		.expect("Failed to save image!");
}

fn create_pixel(r: u8, g: u8, b: u8) -> Pixel {
	return Pixel::new(r, g, b);
}
