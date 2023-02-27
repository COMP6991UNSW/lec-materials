use bmp::Image;

pub fn main() {
    let image = Image::new(256, 256);

    // BEGIN THE HORROR!
    let image = draw_line(image, 80,  100, 56, LineDirection::Vertical);
    let image = draw_line(image, 80,  156, 96, LineDirection::Horizontal);
    let image = draw_line(image, 176, 100, 56, LineDirection::Vertical);
    let image = draw_line(image, 110, 50,  40, LineDirection::Vertical);
    let image = draw_line(image, 146, 50,  40, LineDirection::Vertical);

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
