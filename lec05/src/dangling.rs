struct LinkedList {
    data: i32,
    next: Option<Box<LinkedList>>,
}

fn create_linked_list() -> &'static mut LinkedList {
    let mut list = LinkedList {
        data: 42,
        next: None,
    };

    Box::leak(Box::new(list))
}

