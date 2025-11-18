use q6_lib::RCUType;
use std::thread::{scope, sleep};
use std::time::Duration;

struct NoCopyValue {
    value: i32
}

fn main() {
    scope(|s| {
        let rcu = RCUType::new(0);

        // These threads try and get the first value.
        for _ in 0..2 {
            let rcu_clone = rcu.clone();
            s.spawn(move || {
                let data = rcu_clone.get();
                assert_eq!(*data, 0);
                println!("Checked RCU is 0");

                sleep(Duration::from_millis(3000));

                assert_eq!(*data, 0);
            });
        }

        // Then after a second, we update things
        let rcu_clone = rcu.clone();
        s.spawn(move || {
            let new_value = NoCopyValue { value: 1 };
            assert_eq!(rcu_clone.get_generation(), 0);
            rcu_clone.update(|_| {
                new_value.value
            })
        });

        //  And try and check that.
        for _ in 0..2 {
            let rcu_clone = rcu.clone();
            s.spawn(move || {
                sleep(Duration::from_millis(1000));
                let data = rcu_clone.get();
                assert_eq!(*data, 1);
                println!("Checked RCU is 1");

                sleep(Duration::from_millis(2000));

                assert_eq!(*data, 1);
            });
        }

        // Then after a second, we update things
        let rcu_clone = rcu.clone();
        s.spawn(move || {
            sleep(Duration::from_millis(1500));
            assert_eq!(rcu_clone.get_generation(), 1);
            let new_value = NoCopyValue { value: 2 };
            rcu_clone.update(|_| {
                new_value.value
            })
        });

        //  And try and check that.
        for _ in 0..2 {
            let rcu_clone = rcu.clone();
            s.spawn(move || {
                sleep(Duration::from_millis(2000));
                let data = rcu_clone.get();
                assert_eq!(*data, 2);
                println!("Checked RCU is 2");

                sleep(Duration::from_millis(1000));

                assert_eq!(*data, 2);
                assert_eq!(rcu_clone.get_generation(), 2);
            });
        }
    });

}
