use unsafe_review::RefCell;

fn main() {
    let cell = RefCell::new(String::from("hello"));

    {
        let shared_1 = cell.borrow();
        let shared_2 = cell.borrow();

        println!("I have two strings: {} {}", *shared_1, *shared_2);
    }

    {
        let mut exclusive_1 = cell.borrow_mut();
        exclusive_1.push('!');

        println!("I mutated the string: {}", *exclusive_1);
    }

    {
        println!("But if I try to borrow exclusive and shared simultaneously, it will panic!");

        let _shared_1    = cell.borrow();
        let _exclusive_1 = cell.borrow_mut();
    }
}
