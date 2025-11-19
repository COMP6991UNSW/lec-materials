use std::collections::LinkedList;

fn main() {
    let empty_list:  LinkedList<bool> = linked_list!(X);
    let single_list: LinkedList<&str> = linked_list!(("hi") -> X);
    let multi_list:  LinkedList<i32>  = linked_list!((1) -> (2) -> (3) -> X);

    ::std::io::_print(format_args!("{0:?}\n", empty_list));

    println!("{:?}", empty_list);
    println!("{:?}", single_list);
    println!("{:?}", multi_list);
}

macro_rules! linked_list {
    ($(($item:expr) ->)* X) => {
        {
            let mut list = ::std::collections::LinkedList::new();
            $(
                list.push_back($item);
            )*
            list
        }
    };
}
use linked_list;
