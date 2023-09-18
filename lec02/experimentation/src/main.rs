// fn draw_line(mut image: Image, x: u32, y: u32, length: u32) -> Image {
//     // draw the line
// 
//     image
// }


#[derive(Clone, Copy)]
struct Student {
    x: i32,
    y: f32,
    z: bool,
}

fn main() {

    let x = Option::Some(42);
    x.and_then(|x| None::<String>);

    let n1 = 42;
    let n2 = n1;

    let mut s1 = String::from("hello");
    let mut s2 = s1.clone();
    
    // let image = Image::new(...);
    // let image = draw_line(image, 10, 10, 30);
    // let image = draw_line(image, 10, 10, 30);


    println!("{s1}");
    println!("{s2}");

    // 42;
    // let mut counter = 0;

    // let i = 1;
    // let x = if i != 0 { 42 } else { 10 };

    // let x = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break 42;
    //     }
    // };
}
