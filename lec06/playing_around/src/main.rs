fn main() {
    let mut my_option = Some(42);

    println!("{my_option:?}");

    thief(&mut my_option);

    println!("{my_option:?}");








    let mut number = 42;
    println!("{number}");

    {
        let borrow = &mut number;
        foo(borrow);

        println!("{number}");
    }

    println!("{number}");
}

fn foo(borrow: &mut i32) {
    *borrow = 100;
}

fn thief(pen: &mut Option<i32>) {
    match pen {
        Some(cap) => {
            println!("I've stolen it! {cap}");

            *pen = None;
        }
        None => unreachable!("I know this is not gonna happen"),
    }
}
