fn dereference_null() {
    unsafe {
        *std::ptr::null()
    }
}

fn main() {
    dereference_null();
}
