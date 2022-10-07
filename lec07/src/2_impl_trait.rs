#[derive(Debug)]
struct Binary {
    contents: Vec<bool>,
}

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for bit in &self.contents {
            if *bit {
                write!(f, "1")?;
            } else {
                write!(f, "0")?;
            }
        }
        Ok(())
    }
}

impl std::default::Default for Binary {
    fn default() -> Self {
        Binary::new()
    }
}

impl std::ops::BitAnd for Binary {
    type Output = Binary;

    fn bitand(self, rhs: Binary) -> Self {
        let contents = self
            .contents
            .iter()
            .zip(rhs.contents.iter())
            .map(|(b1, b2)| *b1 && *b2)
            .collect::<Vec<_>>();

        Binary { contents }

    }
}

impl std::convert::From<u8> for Binary {
    fn from(val: u8) -> Self {
        let mut new_contents = vec![];
        for i in 0..8 {
            let mask = 0x1 << i;
            new_contents.push((val & mask) != 0);
        }
        Binary { contents: new_contents }
    }
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

    /// Creates a struct to represent a binary number.
    fn new() -> Binary {
        Binary { contents: vec![] }
    }


}


fn main() {
    let my_binary1: Option<Binary> = None;
    let mut my_binary1 = my_binary1.unwrap_or_default();
    let mut my_binary2: Binary = Binary::from(128);
    my_binary1.reset(126);
    println!("B1: {my_binary1}");
    println!("B2: {my_binary2}");
    let my_binary3 = my_binary1 & my_binary2;
    let bools = my_binary3.to_bools();
    println!("B1 & B2: {bools:?}");

}
