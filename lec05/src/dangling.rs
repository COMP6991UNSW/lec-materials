struct LinkedList {
    data: i32,
    next: Option<Box<LinkedList>>,
}

static NODE: LinkedList = LinkedList {
    data: 42,
    next: None,
};

fn create_linked_list(data: i32) -> &'static LinkedList {
    // let list = LinkedList {
    //     data,
    //     next: None,
    // };

    &NODE
}

fn create_linked_list_cheeky(data: i32) -> &'static LinkedList {
    let list = Box::new(LinkedList {
        data,
        next: None,
    });

    let leaked: &'static LinkedList = Box::leak(list);
    leaked
}
