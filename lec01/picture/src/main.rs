use bmp::{Image, Pixel};
use std::cmp::Ordering;
mod enums;
use enums::Test::A;

fn main() {
    let width = 100;
    let mut img = Image::new(width, 256);

    let red: Pixel = Pixel {
        r: 100,
        g: 10,
        b: 0,
    };
    let green = Pixel::new(0, 200, 0);
    let blue = Pixel::new(0, 0, width.try_into().unwrap());

    for x in 0..=255 {
        for y in 0..=255 {
            match x.cmp(&y) {
                Ordering::Less => {
                    img.set_pixel(x, y, red);
                }
                Ordering::Equal => {
                    img.set_pixel(x, y, green);
                }
                Ordering::Greater => {
                    img.set_pixel(x, y, red);
                }
            }
        }
    }

    if let Err(e) = img.save("img.bmp") {
        println!("Errored: {e}")
    }
}

fn add_u32(x: u16) -> Result<u32, ()> {
    Ok(x + 1u16).map(|x| x as u32)
}

fn add_u16(x: u16) -> Result<u16, ()> {
    Ok(x + 1)
}
