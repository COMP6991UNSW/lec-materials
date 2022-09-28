//! Source: The Book

// 'b: 'a

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
// 'b     :      'a

fn ok() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;        // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
