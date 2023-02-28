//! # Source: The Book

// borrowck

// 'a "outlives" 'b
// i.e. 'a: 'b
#[cfg(remove_me)]
fn dangling() {
    let r;          // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;   // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   // ---------+
}

















// 'b "outlives" 'a
// i.e. 'b: 'a
fn ok() {
    let x = 5;       // ----------+-- 'b
                          //           |
    let r = &x;     // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+