use std::collections::HashMap;
use exam_q6_lib::{get_factors, get_common_factors};

fn main() {
    let args: Vec<u128> = std::env::args().skip(1).map(|x| x.parse().unwrap()).collect();

    let mut factors: HashMap<u128, Vec<u128>> = HashMap::new();
    for arg in args {
        factors.insert(arg, get_factors(arg));
    }

    let mut common_factors: HashMap<(u128, u128), Vec<u128>> = HashMap::new();
    for (val1, facs1) in &factors {
        for (val2, facs2) in &factors {
            if val1 > val2 {
                common_factors.insert((*val1, *val2), get_common_factors(&facs1, &facs2));
            }
        }
    }

    let mut keys = common_factors.keys().collect::<Vec<_>>();
    keys.sort();

    for k in keys {
        println!("{k:?}: {:?}", common_factors[k]);
    }
}
