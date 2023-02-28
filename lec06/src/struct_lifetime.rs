struct Student<'a> {
    name: &'a str,
    zid: u32,
    wam: Option<f64>,
}

fn student_name<'a>(student: &Student<'a>) -> &'a str {
    student.name
}