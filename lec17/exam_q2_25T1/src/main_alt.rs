use exam_q2_lib::split_many_2;

fn main() {
    let inputs = vec!["foo->bar->baz".to_string(), "x->y->z".to_string()];
    let result;

    {
        let arrow = "->";
        result = split_many_2(&inputs, arrow);

        for (a, mid, b) in &result {
            println!("a: {}, mid: {}, b: {}", a, mid, b);
        }
    }

    let result2;
    {
        let arrow = "->".to_string();
        result2 = split_many_2(&inputs, &arrow);
    }
    for (a, mid, b) in result2 {
        println!("a: {}, mid: {}, b: {}", a, mid, b);
    }
}
