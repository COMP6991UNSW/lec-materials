use std::time::{Duration, Instant};

pub trait MyMapExt<I> {
    fn my_map<F, CurrItem, NewItem>(self, function: F) -> MyMap<I, F>
    where
        I: Iterator<Item = CurrItem>,
        F: FnMut(CurrItem) -> NewItem;
}

impl<I> MyMapExt<I> for I
where
    I: Iterator
{
    fn my_map<F, CurrItem, NewItem>(
        self,
        function: F
    ) -> MyMap<I, F>
    {
        MyMap {
            iter: self,
            function,
        }
    }
}



pub struct MyMap<I, F> {
    iter: I,
    function: F,
}

impl<I, F, CurrItem, NewItem> Iterator for MyMap<I, F>
where
    I: Iterator<Item = CurrItem>,
    F: FnMut(CurrItem) -> NewItem,
{
    type Item = NewItem;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_item = self.iter.next()?;
        let new_item = (self.function)(curr_item);
        Some(new_item)
    }
}

fn time_closure<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let before = Instant::now();
    let t = f();
    (t, Instant::now().duration_since(before))
}

fn main() {
    // Closure: fn-pointer + environment (captures)

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let greeting = line.trim().to_string();
    

    

    // My map

    let mut nth = 1;

    let names = vec!["Miguel", "Mitchell", "Patrick", "Dong", "Tom"];
    let greeted = names.into_iter()
        .my_map(|name| {
            let greet_name = format!("#{nth}: {greeting} {name}");
            nth += 1;

            greet_name
        })
        .collect::<Vec<_>>();
    
    println!("My map: {greeted:?}");





    // Std

    let mut nth = 1;

    let names = vec!["Miguel", "Mitchell", "Patrick", "Dong", "Tom"];
    let greeted = names.into_iter()
        .map(|name| {
            let greet_name = format!("#{nth}: {greeting} {name}");
            nth += 1;

            greet_name
        })
        .collect::<Vec<_>>();
    
    println!("Std: {greeted:?}");


    // let x = |x: i32, y: i32| -> f32 { (x + y) as f32 * factor };
    // foo(x);
}

/*

    struct MainXClosureCaptures<'a> {
        borrow_factor: &'a f32
    }

    impl Fn(i32, i32) -> f32 for MainXClosureCaptures<'_> {
        fn call(&self, x: i32, y: i32) -> f32 {
            (x + y) as f32 * self.borrow_factor
        }
    }
    let x = MainXClosureCaptures { borrow_factor: &factor };





    struct MainXClosureCaptures<'a> {
        borrow_factor: &'a mut f32
    }

    impl FnMut(i32, i32) -> f32 for MainXClosureCaptures<'_> {
        fn call(&mut self, x: i32, y: i32) -> f32 {
            (x + y) as f32 * self.borrow_factor
        }
    }
    let x = MainXClosureCaptures { borrow_factor: &mut factor };



    struct MainXClosureCaptures {
        borrow_factor: f32
    }

    impl FnOnce(i32, i32) -> f32 for MainXClosureCaptures<'_> {
        fn call(self, x: i32, y: i32) -> f32 {
            (x + y) as f32 * self.borrow_factor
        }
    }
    let x = MainXClosureCaptures { borrow_factor: factor };

*/
