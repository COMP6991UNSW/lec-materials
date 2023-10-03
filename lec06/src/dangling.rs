struct LinkedList {
    data: i32,
    next: Option<Box<LinkedList>>,
}

fn create_linked_list() -> &'static mut LinkedList {
    let list = Box::leak(Box::new(LinkedList {
        data: 42,
        next: None,
    }));

    list
}

