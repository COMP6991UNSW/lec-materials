use unsafe_review::Cell;

fn main() {
    let mut cell = Cell::new(10);
    println!("Initial: {}", cell.get());

    cell.set(20);
    println!("After set: {}", cell.get());

    let old = cell.replace(30);
    println!("After replace: {}, got back: {old}", cell.get());

    let r = cell.get_ref();
    println!("Via get_ref: {r}");

    let m = cell.get_mut();
    *m = 99;
    println!("Via get_mut: {}", cell.get());
}
