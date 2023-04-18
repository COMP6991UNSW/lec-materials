use std::{collections::HashMap, thread, sync::Mutex};
use exam_q6_lib::{get_factors, get_common_factors};

fn main() {
    let args: Vec<u128> = std::env::args().skip(1).map(|x| x.parse().unwrap()).collect();

    let factors: Mutex<HashMap<u128, Vec<u128>>> = Mutex::new(HashMap::new());
    thread::scope(|scope| {
        for arg in args {
            let factors = &factors;
            scope.spawn(move || {
                let calculated = get_factors(arg);

                factors.lock().unwrap().insert(arg, calculated);
            });
        }
    });

    let factors = factors.into_inner().unwrap();

    let common_factors: Mutex<HashMap<(u128, u128), Vec<u128>>> = Mutex::new(HashMap::new());
    thread::scope(|scope| {
        for (val1, facs1) in &factors {
            for (val2, facs2) in &factors {
                if val1 > val2 {
                    let common_factors = &common_factors;
                    scope.spawn(move || {
                        common_factors.lock().unwrap().insert((*val1, *val2), get_common_factors(&facs1, &facs2));
                    });
                }
            }
        }
    });

    let common_factors = common_factors.into_inner().unwrap();

    let mut keys = common_factors.keys().collect::<Vec<_>>();
    keys.sort();

    for k in keys {
        println!("{k:?}: {:?}", common_factors[k]);
    }
}
