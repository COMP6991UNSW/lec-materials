use my_slick_lib::add;
use rand::random;

fn main() {
    dbg!(add(42, random()));
}
