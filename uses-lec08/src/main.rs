use lec08::{
    add_two,
    inline_module::foo as f,
};

fn main() {
    let sum = add_two(10, 15);
    println!("{sum}");

    println!("{}", lec08::folder_module::function_from_a());

    let foo = lec08::Foo::new(42);

    foo.foo();

    f();
}
