use exam_q2_lib::{compare_rolls, DiffResult};

fn get_string_until_empty_line() -> String {
    std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.starts_with('#'))
        .map(|l| format!("{l}\n"))
        .take_while(|l| !l.trim().is_empty())
        .collect()
}

fn main() {
    let tutors_start_of_term = get_string_until_empty_line();
    let tutors_end_of_term = get_string_until_empty_line();

    let end_of_term_only = {
        let comparison = compare_rolls(&tutors_start_of_term, &tutors_end_of_term);

        let right_onlys = comparison
            .into_iter()
            .filter_map(|c| {
                if let DiffResult::RightOnly(l) = c {
                    Some(l)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        
        drop(tutors_start_of_term);

        right_onlys
    };

    for student in end_of_term_only {
        println!("Right Only: {}", student);
    }

}
