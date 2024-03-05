// struct student {
//     char *stu_name;
//     int stu_zid;
//     double stu_wam;
// }
// 
// fn prints_student(student: struct student) {
//     // ...
// }

fn sum_vec(xs: Vec<i32>) -> i32 {
    xs.into_iter().sum()
}

// fn (Rust)
// fun (Kotlin)
// func (Go)
// function (JS)
// def (Python)
// sub (Perl)
// <nothing> (Shell)
//
// int x = 42;      --> <TYPE(int)> <VAR(x)> <EQ> <NUM(42)> <SEMICOLON>
// let x: i32 = 42; --> <LET> <VAR(x)> <TYPE(i32)> <EQ> <NUM(42)> <SEMICOLON>
//                      ^^^^ parser ??? assignment 01 ?????????

fn sum_array(xs: [i32; 3]) -> i32 {
    xs.into_iter().sum()
}

fn sum_slice(xs: &[i32]) -> i32 {
    xs.into_iter().sum()
}

// fn foo(x: i32) {
//     println!("Foo with i32: {x}");
// }
// 
// fn foo(x: &str) {
//     println!("Foo with string: {x}");
// }

fn main() {
    let v = vec![1, 2, 3];
    let a = [1, 2, 3];

    // ...

    dbg!(sum_slice(&v));
    dbg!(sum_slice(&a));
}
