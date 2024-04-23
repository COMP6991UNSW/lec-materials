use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

macro_rules! arc_mutex {
    ($($name:ident = $value:expr ;)+) => {
        $(
            let $name = Arc::new(Mutex::new($value));
        )+
    };
}

macro_rules! use_arc_mutex {
    ($($name:ident ;)* $body:block) => {
        {
            $(
                let $name = $name.clone();
            )*

            $body
        }
    };
}

fn main() {
    arc_mutex!(
        a = 42;
        b = "hello";
        c = 3.14;
    );

    use_arc_mutex!(
        a;
        b;
        {
            thread::spawn(move || {
                println!("{} {}", a.lock().unwrap(), b.lock().unwrap());
            }).join().unwrap();
        }
    );

    use_arc_mutex!(
        b;
        c;
        {
            thread::spawn(move || {
                println!("{} {}", b.lock().unwrap(), c.lock().unwrap());
            }).join().unwrap();
        }
    );
}
