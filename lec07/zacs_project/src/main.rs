use bmp::Image;
use lec07::add;
use lec07::students::Student;
use lec07::students::some_student_fn;
use bmp::consts::DARK_GRAY;

fn main() {
    let my_number = add(2, 3);
    println!("Number is {my_number}");
    some_student_fn();

    let mut student = Student::new(
        String::from("stu"),
        5555555,
    );
    student.set_wam(50.0);
    student.set_wam(5000.0);
    println!(
        "Student has name {} zid {} wam {:?}",
        student.name,
        student.zid,
        student.wam(),
    );

}
