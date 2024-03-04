use mylib::my_favourite_number;

mod a;
mod b;
mod util;
mod foo;
mod bar;

fn main() {
    let the_number = my_favourite_number();

    println!("Hello, world!");

    let student = hello::foo();
    let name = student.name();

    println!("{name}");

    let x = 42;
    let y = util::add_5(x);
    dbg!(y);

    foo::a::hello();
    bar::b::hello();
}

pub mod hello {
    pub fn foo() -> Student {
        println!("foo");

        Student {
            name: String::from("foo"),
            zid: 5555555,
            wam: None,
        }
    }

    pub struct Student {
        name: String,
        zid: u32,
        wam: Option<f64>,
    }

    impl Student {
        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn name_mut(&mut self) -> &mut str {
            &mut self.name
        }

        // pub fn set_name(&mut self, new_name: String) {
        //     self.name = new_name;
        // }

        pub fn zid(&self) -> u32 {
            self.zid
        }
    }

    mod world {

    }
}