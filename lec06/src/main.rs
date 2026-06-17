struct Student {
    name: String,
    zid: u32,
    wam: Option<f64>,
}

struct MyOwnedValue {
    items: [i64; 1024],
}

fn does_something_with_a_student(student: &mut Student) {

}

fn takes_shared_borrow(x: &i32) {}

fn sum_list(xs: &[i32]) -> i32 {
    let mut sum = 0;
    for x in xs {
        sum += x;
    }
    sum
}

fn slices() {
    let array =   [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];
    
    sum_list(&array);
    sum_list(vec.as_slice());
    sum_list(&vec[..]);
    
    // bare slice: [T]
    
    let sub_slice = &vec[1..4];
    dbg!(sum_list(sub_slice));
    let sb1: &[i32] = &array; // (5, ptr)
    let sb2: &[i32] = &vec;   // (5, ptr)
    let first = sb1.first();
    let sb1 = array.as_slice();
    let sb2 = vec.as_slice();


    let string_1 = "hello"; // (5, ptr)
    let string_2 = String::from("hello");
    let sb3: &str = &string_2;
    let sb3 = string_2.as_str();
    println!("{string_1}");
}




fn main() {
    slices();

    let mut x = 42i32;
    let borrow = &mut x;
    let shared = &(*borrow);
    takes_shared_borrow(borrow);

    println!("Hello, world!");

    let mut student = Student {
        name: String::from("stu"),
        zid: 5555555,
        wam: None,
    };

    let name = &mut (student.name);
    // does_something_with_a_student(&mut student);
    println!("{name}");
}
