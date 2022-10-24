#![allow(unused)]

mod c_macros;
mod rust_macros;

fn main() {
    c_macros::max::test_max();
    c_macros::up_to::test_up_to();
    c_macros::sort::test_sort();

    rust_macros::vec::test_vec();
    rust_macros::c_for::test_c_for();
}
