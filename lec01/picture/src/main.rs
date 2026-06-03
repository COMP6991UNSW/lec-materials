use bmp::{Image, Pixel};

const IMAGE_SIZE_PX: u32 = 256;

fn main() {
    let mut image = Image::new(IMAGE_SIZE_PX, IMAGE_SIZE_PX);

    let red = Pixel::new(255, 0, 0);
    let green = Pixel::new(0, 255, 0);
    let blue = Pixel::new(0, 0, 255);
    let black = Pixel::new(0, 0, 0);

    for x in 0..IMAGE_SIZE_PX {
        for y in 0..IMAGE_SIZE_PX {
            let colour = match (x + y) % 15 {
                0  => red,
                5  => green,
                10 => blue,
                _  => black,
            };
            image.set_pixel(x, y, colour);
        }
    }

    image.save("lines.bmp")
        .expect("failed to save image");
}





















//    let red = Pixel {
//        r: 255,
//        g: 0,
//        b: 0,
//    };
//
//          if (x + y) % 15 == 0 {
//              image.set_pixel(x, y, red);
//          } else if (x + y) % 15 == 5 {
//              image.set_pixel(x, y, green);
//          } else if (x + y) % 15 == 10 {
//              image.set_pixel(x, y, blue);
//          } else {
//              image.set_pixel(x, y, black);
//          }
//
//          Image::set_pixel(&mut image, x, y, colour);
