#![allow(unused)]

use std::time::{Duration, Instant};

struct MyMap<I, F, CurrItem, NextItem>
where
    I: Iterator<Item = CurrItem>,
    F: FnMut(CurrItem) -> NextItem,
{
    iter: I,
    function: F,
}

impl<I, F, CurrItem, NextItem> Iterator for MyMap<I, F, CurrItem, NextItem>
where
    I: Iterator<Item = CurrItem>,
    F: FnMut(CurrItem) -> NextItem,
{
    type Item = NextItem;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        let mapped = (self.function)(item);

        Some(mapped)
    }
}


/*
 * <---- foo ----->      <----- foo ----->
 * <---- foo ----->      <----- foo ----->
 *         <----- foo ----->
 * <---- foo ----->      <----- foo ----->
 * <---- foo ----->      <----- foo ----->
 * <---- foo ----->      <----- foo ----->
 *                 <----- foo ----->
 * <---- foo ----->      <----- foo ----->
 *                 <----- foo ----->
 * <---- foo ----->      <----- foo ----->
 *                 <----- foo ----->
 */

fn my_map<I, F, CurrItem, NextItem>(
    iter: I,
    function: F,
) -> MyMap<I, F, CurrItem, NextItem>
where
    I: Iterator<Item = CurrItem>,
    F: FnMut(CurrItem) -> NextItem,
{
    MyMap {
        iter,
        function,
    }
}


fn time_function<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let return_value = f();
    let after = Instant::now();

    let duration = after.duration_since(start);

    (return_value, duration)
}


#[cfg(test)]
mod tests {
    use super::*;

    // struct TimeFunctionClosure {
    //     x: Vec<i32>,
    // }

    // impl FnMut for TimeFunctionClosure {
    //     fn call_mut(&mut self, args: Args) -> Self::Output {
    //         let y = &self.x;
    //         std::thread::sleep(Duration::from_millis(5));
    //         42
    //     }
    // }


    #[test]
    fn test_time_function() {
        let (ret, duration) = time_function(|| {
            std::thread::sleep(Duration::from_millis(5));
            42
        });

        println!("Function took {duration:?} and returned {ret}");
    }

    #[test]
    fn it_works() {
        fn times_two(x: i32) -> i32 {
            x * 2
        }

        let a = [1, 2, 3];
        let iter = my_map(a.into_iter(), times_two);

        assert_eq!(vec![2, 4, 6], iter.collect::<Vec<_>>());
    }

    #[test]
    fn map_across_types() {
        let a = [1, 2, 3];
        let iter = my_map(a.into_iter(), |x| x.to_string());

        assert_eq!(
            vec![String::from("1"), String::from("2"), String::from("3")],
            iter.collect::<Vec<_>>()
        );
    }

    #[test]
    fn map_with_env() {
        let factor;

        // We don't know what happens in here
        {
            factor = 3;
        }

        let a = [1, 2, 3];
        let iter = my_map(a.into_iter(), |x| x * factor);

        assert_eq!(vec![3, 6, 9], iter.collect::<Vec<_>>());
    }

    #[test]
    fn map_with_mut_env() {
        let factor;

        // We don't know what happens in here
        {
            factor = 3;
        }

        let mut count = 0;

        let a = [1, 2, 3];
        let iter = my_map(a.into_iter(), |x| {
            count += 1;
            x * factor
        });

        assert_eq!(vec![3, 6, 9], iter.collect::<Vec<_>>());
        assert_eq!(count, 3);
    }

    #[test]
    fn map_with_mut_env_std() {
        let factor;

        // We don't know what happens in here
        {
            factor = 3;
        }

        let mut count = 0;

        let a = [1, 2, 3];
        let iter = a.into_iter().map(|x| {
            count += 1;
            x * factor
        });

        assert_eq!(vec![3, 6, 9], iter.collect::<Vec<_>>());
        assert_eq!(count, 3);
    }

    /*
    #[test]
    fn map_with_env_fn_trait_example() {
        let factor;

        // We don't know what happens in here
        {
            factor = 3;
        }

        let mut count = 0;

        struct TestLine93Closure<'a> {
            count: &'a mut i32,
            factor: i32,
        }

        impl<'a> Fn<i32> for TestLine93Closure<'a> {
            type Output = i32;

            fn call(&self, args: Args) -> Self::Output {
                self.count += 1;
                args.x * self.factor
            }
        }

        let a = [1, 2, 3];
        let iter = my_map(a.into_iter(), TestLine93Closure { count: &mut count, factor });

        assert_eq!(vec![3, 6, 9], iter.collect::<Vec<_>>());
    }
    */
}
