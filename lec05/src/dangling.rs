struct LinkedList {

}

fn create_linked_list() -> &'static LinkedList {
    let list = LinkedList {  };

    Box::leak(Box::new(list))
}
