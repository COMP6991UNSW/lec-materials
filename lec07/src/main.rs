use my_lib::add;
use my_lib::Point;
use my_lib::Image;

use my_lib::my_folder::b::B;

fn main() {
    let answer = add(2, 2);

    my_lib::my_folder::hello();

    println!("hello from binary crate! 2 + 2 = {answer}");

    let point1 = Point::new(1,2).unwrap();
    let point2 = Point::new(3, 4).unwrap();

    println!("Point 1 is ({}, {})", point1.x(), point1.y());

    let point3 = point1.add(point2);
}
