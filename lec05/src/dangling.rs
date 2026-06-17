struct LinkedList {
    data: i32,
    next: Option<Box<LinkedList>>,
}

static LIST: LinkedList = LinkedList {
    data: 42,
    next: None,
};

fn create_linked_list(data: i32) -> Box<LinkedList> {
    Box::new(LinkedList {
        data,
        next: None,
    })
}
