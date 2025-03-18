struct Crab {
    name: String,
    shell_count: i32,
    has_shell: bool,
}

struct Mouse {
    name: String,
    age: i32,
    cheese: bool,
}
impl Speak for Mouse {
    fn speak(&self) -> String {
        String::from("Squeak!")
    }
}

struct Cow {
    name: String,
    age: i32,
    is_mooing: bool,
}

impl Speak for Cow {
    fn speak(&self) -> String {
        String::from("MOooooo!")
    }
}

trait Speak {
    fn speak(&self) -> String;
}

fn speak_to_eachother(animals: Vec<Box<dyn Speak>>) {
    for a in animals {
        dbg!(a.speak());
    }
}

/* Not extesible!
enum Animal {
    Cow(Cow),
    Mouse(Mouse),
}*/

fn main() {
    let c1 = Box::new(Cow {
        name: String::from("Tom"),
        age: 100,
        is_mooing: true,
    });
    let m2 = Box::new(Mouse {
        name: String::from("Mot"),
        age: 100,
        cheese: true,
    });
    dbg!(speak_to_eachother(vec![c1, m2]));

    println!("Hello, world!");
}

trait ExistsIn2DSpace {}
trait Movable {}

impl<T> Add for T {}

impl Add for Coordinate {}

mod new_lib {
    trait IsInASpace {}

    impl<T: IsInASpace> Add for T {}
}
