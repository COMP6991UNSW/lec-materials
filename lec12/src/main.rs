use std::{thread::sleep, time::{Duration, Instant}};

fn my_map<I, F, CurrItem, NewItem>(
    iter: I,
    f: F,
) -> MyMap<I, F>
where
    I: Iterator<Item = CurrItem>,
    F: FnMut(CurrItem) -> NewItem,
{
    MyMap {
        iter,
        f,
    }
}

trait MyMapExt: Iterator {
    fn my_map<F, NewItem>(
        self,
        f: F,
    ) -> MyMap<Self, F>
    where
        F: FnMut(<Self as Iterator>::Item) -> NewItem,
        Self: Sized;
}

impl<I> MyMapExt for I
where
    I: Iterator,
{
    fn my_map<F, NewItem>(
        self,
        f: F,
    ) -> MyMap<Self, F>
    where
        F: FnMut(<Self as Iterator>::Item) -> NewItem,
    {
        MyMap {
            iter: self,
            f,
        }
    }
}

struct MyMap<I, F> {
    iter: I,
    f: F,
}

impl<I, F, CurrItem, NewItem> Iterator for MyMap<I, F>
where
    I: Iterator<Item = CurrItem>,
    F: FnMut(CurrItem) -> NewItem,
{
    type Item = NewItem;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_item = self.iter.next()?;
        let new_item = (self.f)(curr_item);

        Some(new_item)
    }
}

fn takes_owned_string(s: String) {
    // ...
}

fn time_function<R>(f: impl FnOnce() -> R) -> R {
    let before = Instant::now();
    
    let ret = f();
    
    let elapsed = before.elapsed();
    println!("Function took {elapsed:?}");

    ret
}

fn main() {
    let s = String::from("hi");
    let x = time_function(|| {
        takes_owned_string(s);
        sleep(Duration::from_secs(2));

        42
    });
    println!("x={x}");

    let mut user_num = get_user_input();

    dbg!([1, 20, 300, 4000].into_iter()
        .map(|x| {
            println!("called!");
            user_num += 1;
            (x * user_num).to_string()
        })
        .filter(|s| s.len() > 2)
        .collect::<Vec<_>>());

    let mut user_num = get_user_input();

    dbg!([1, 20, 300, 4000].into_iter()
        .my_map(|x| {
            println!("called!");
            user_num += 1;
            (x * user_num).to_string()
        })
        .filter(|s| s.len() > 2)
        .collect::<Vec<_>>());

    // let i = [1, 2, 3].into_iter();
    // my_map(i, |x| x * 2)
    //     .collect::<Vec<_>>();
}

fn get_user_input() -> i32 {
    3
}
















fn call_fn_n_times(n: u32, f: fn()) {
    for _ in 0..n {
        f();
    }
}
