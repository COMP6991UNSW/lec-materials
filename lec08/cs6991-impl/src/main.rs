use std::ops::Add;

#[derive(Copy, Clone, Debug)]
struct Complex {
    real: i32,
    imaginary: i32,
}

impl Default for Complex {
    fn default() -> Self {
        Self { real: 0, imaginary: 0 }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}


impl Complex {
    fn add(self, other: Self) -> Self {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary
        }
    }
}



fn add(x: Complex, y: Complex) -> Complex {
    Complex {
        real: x.real + y.real,
        imaginary: x.imaginary + y.imaginary
    }
}

fn main() {
    let the_default = Complex::default();

    let x = Complex { real: 42, imaginary: 123 };
    let y = Complex { real: 314, imaginary: 6991 };

    dbg!(add(x, y));
    dbg!(x.add(y));
    dbg!(x + y);
}
