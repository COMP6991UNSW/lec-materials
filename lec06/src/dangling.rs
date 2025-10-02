//! Source: The Book

#[cfg(never)]
fn dangling() {
    
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   // ---------+
}














fn ok() {
    let x = 5;       // ----------+-- 'b
                          //           |
    let r = &x;     // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
