fn main() {
    let mut v = vec![1, 2, 3];
    // Safety:
    // new_len must be less than or equal to capacity().
    // - ...
    //
    // The elements at old_len..new_len must be initialized.
    // - ...
    unsafe { v.set_len(7); }
}
