fn main() {
    let xs = vec![1, 4, 5, 7, 13];
    let mean = find_the_mean(&xs);
    println!("The mean of {xs:?} is: {mean:?}");
}

fn find_the_mean(xs: &Vec<i32>) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }

    let size = xs.len();

    let mut sum = 0;
    for x in xs {
        sum += x;
    }

    let mean = (sum as f64) / (size as f64);
    
    Some(mean)
}
