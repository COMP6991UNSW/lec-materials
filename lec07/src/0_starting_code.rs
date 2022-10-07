#[derive(Debug)]
struct Binary {
    contents: Vec<bool>,
}



/// Creates a struct to represent a binary number.
fn create_binary_number() -> Binary {
    Binary { contents: vec![] }
}

/// Converts the struct into a list of bools representing 0s and 1s.
fn to_bools(binary: Binary) -> Vec<bool> {
    binary.contents
}

// Sets a [`Binary`] struct to represent whatever
// provided `num` is.
fn set_binary_number(binary: &mut Binary, num: u8) {
    let mut new_contents = vec![];
    for i in 0..8 {
        let mask = 0x1 << i;
        new_contents.push((num & mask) != 0);
    }
    binary.contents = new_contents;
}

// binary & other
fn bitwise_and(binary: &Binary, other: &Binary) -> Binary {
    let contents = binary
        .contents
        .iter()
        .zip(other.contents.iter())
        .map(|(b1, b2)| *b1 && *b2)
        .collect::<Vec<_>>();

    Binary { contents }
}

fn main() {
    let mut my_binary1 = create_binary_number();
    let mut my_binary2 = create_binary_number();
    set_binary_number(&mut my_binary1, 126);
    set_binary_number(&mut my_binary2, 127);
    println!("B1: {my_binary1:?}");
    println!("B2: {my_binary2:?}");
    let my_binary3 = bitwise_and(&my_binary1, &my_binary2);
    let bools = to_bools(my_binary3);
    println!("B1 & B2: {bools:?}");

}
