#[derive(Debug)]
struct Binary {
    contents: Vec<bool>,
}

impl Binary {
    /// Converts the struct into a list of bools representing 0s and 1s.
    fn to_bools(self) -> Vec<bool> {
        self.contents
    }

    // Sets a [`Binary`] struct to represent whatever
    // provided `num` is.
    fn reset(&mut self, num: u8) {
        let mut new_contents = vec![];
        for i in 0..8 {
            let mask = 0x1 << i;
            new_contents.push((num & mask) != 0);
        }
        self.contents = new_contents;
    }

    // binary & other
    fn bitwise_and(&self, other: &Binary) -> Binary {
        let contents = self
            .contents
            .iter()
            .zip(other.contents.iter())
            .map(|(b1, b2)| *b1 && *b2)
            .collect::<Vec<_>>();

        Binary { contents }
    }

    /// Creates a struct to represent a binary number.
    fn new() -> Binary {
        Binary { contents: vec![] }
    }


}


fn main() {
    let mut my_binary1 = Binary::new();
    let mut my_binary2 = Binary::new();
    my_binary1.reset(126);
    my_binary2.reset(127);
    println!("B1: {my_binary1:?}");
    println!("B2: {my_binary2:?}");
    let my_binary3 = my_binary1.bitwise_and(&my_binary2);
    let bools = my_binary3.to_bools();
    println!("B1 & B2: {bools:?}");

}
