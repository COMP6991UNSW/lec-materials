use bmp::Image;

pub fn main() {
    let image = Image::new(256, 256);
    let image = draw_line(image, 20, 20, 80, LineDirection::Horizontal);

    image.save("my_image.bmp").unwrap();
}

enum LineDirection {
    Vertical,
    Horizontal,
}

fn draw_line(mut image: Image, x: u32, y: u32, length: u32, direction: LineDirection) -> Image {
    for i in 0..length {
        let (cur_x, cur_y) = match direction {
            LineDirection::Horizontal => {
                (x + i, y)
            }
            LineDirection::Vertical => {
                (x, y + i)
            }
        };

        image.set_pixel(cur_x, cur_y, bmp::consts::RED);
    }

    image
}
