struct Student {
    name: String,
    zid: u32,  // 5257261
    wam: Option<f64>,
}

enum Foo {
    A(i32),
    B(String, bool),
    C {
        a: i32,
        b: String,
        c: bool,
    },
}

fn main() {
    let my_foo = Foo::B(String::from("hello"), true);
    match my_foo {
        Foo::A(_) => {
            println!("it was Foo::A");
        }
        Foo::B(str, b) => {
            println!("It was B, it had {str} and {b} inside it!");
        }
        Foo::C { a, b, c } => todo!(),
    }


    let mut xs = [1, 2, 3];
    let mut xs = vec![1, 2, 3];
    let mut xs = Vec::new();
    xs.push(1);
    xs.push(2);
    xs.push(3);
    xs.remove(2);
    xs.remove(1);
    xs.remove(0);

    let point: (i32, i32) = (3, 4);
    let student = ("Zac", 5257261, Some(63.5));
    let a = student.0;
    let a = student.1;
    let a = student.2;
    
    let (name, zid, wam) = student;

    let x = 50;
    let y = 25;
    let (x, y) = (y, x);

    let unit = ();

    let student = Student {
        name: String::from("Zac"),
        zid: 5257261,
        wam: Some(63.5),
    };

    let mut mut_student = Student {
        name: String::from("foo"),
        zid: 5555555,
        wam: None,
    };

    let zid = student.zid;
    mut_student.zid = 5257262;


    let x = 42;

    {
        let x = "hello";
        println!("{x}");
    }

    println!("{x}");
}
