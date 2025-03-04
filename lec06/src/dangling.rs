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
    println!("r: {}", r); // ---------+
}














// 'b "outlives" 'a
//
// a.k.a.:
// 'b     :      'a
//
// 'b: 'a
//
//
// 'a, 'b, 'c
//
// 'a: 'c
// 'b: 'c
//

fn ok() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
