macro_rules! cfor {
    (for ($init:stmt; $cond:expr; $step:stmt) $body:block) => {
        $init
        while ($cond) {
            $body
            $step
        }
    }
}

pub fn test_c_for() {
    cfor! {
        for (let mut i = 0; i < 10; i += 1) {
            println!("{i}");
        }
    }
}
