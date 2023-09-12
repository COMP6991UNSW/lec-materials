use bmp::{Pixel, Image};

const IMAGE_SIZE_PX: u32 = 256;

struct Student {
    name: String,
    zid: u32,
    wam: f64,
    is_comp_student: bool,
}

fn main() {
    // let my_array = [1, 2, 3, 4];
    // let my_tuple: (i32, &str, bool, f64) = (42, "hello", false, 3.14);

    let mut my_image = Image::new(IMAGE_SIZE_PX, IMAGE_SIZE_PX);

    let red = create_pixel(255, 0, 0);
    let green = create_pixel(0, 255, 0);
    let blue = create_pixel(0, 0, 255);
    let white = create_pixel(255, 255, 255);
    
    for x in 0..IMAGE_SIZE_PX {
        for y in 0..IMAGE_SIZE_PX {
            let pixel = if (x + y) % 15 == 0 {
                red
            } else if (x + y) % 15 == 5 {
                green
            } else if (x + y) % 15 == 10 {
                blue
            } else {
                white
            };

            my_image.set_pixel(
                x,
                y,
                pixel
            );

            // if (x + y) % 15 == 0 {
            //     my_image.set_pixel(x, y, red);
            // } else if (x + y) % 15 == 5 {
            //     my_image.set_pixel(x, y, green);
            // } else if (x + y) % 15 == 10 {
            //     my_image.set_pixel(x, y, blue);
            // } else {
            //     my_image.set_pixel(x, y, white);
            // }
        }
    }

    my_image.save("my_image.bmp").expect("Failed to save image!");
}

fn create_pixel(r: i32, g: i32, b: i32) -> Pixel {
    // let pixel = Pixel {
    //     r: r.try_into().expect("i32 was too big"),
    //     g: g.try_into().expect("i32 was too big"),
    //     b: b.try_into().expect("i32 was too big"),
    // };

    // return pixel;

    Pixel {
        r: r.try_into().expect("i32 was too big"),
        g: g.try_into().expect("i32 was too big"),
        b: b.try_into().expect("i32 was too big"),
    }
}
