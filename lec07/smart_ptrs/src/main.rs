struct LinkedList<T> {
    head: Node<T>,
}

// enum Node<T> {
//     HasNext(T, Node),
//     Tail(T),
// }

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

fn main() {

}
