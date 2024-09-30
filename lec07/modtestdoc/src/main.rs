pub mod d;

use modtestdoc::a::add_2;
use my_library::my_awesome_fn;

use modtestdoc::hello::Foo;
use itertools::Batching;

use d::foo;

fn main() {
    let foo = Foo::new();
    // foo.y;

    let result = add_2(10, 5);
    
    println!("{result}");

    my_awesome_fn();

    modtestdoc::a::b::foo();

    d::foo();
}