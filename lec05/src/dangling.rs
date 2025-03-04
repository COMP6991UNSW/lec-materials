struct LinkedList {
    data: i32,
    next: Option<Box<LinkedList>>,
}

fn create_linked_list(data: i32) -> Box<LinkedList> {
    let list = LinkedList {
        data,
        next: None,
    };

    Box::new(list)
}
