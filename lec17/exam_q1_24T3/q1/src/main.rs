fn main() {
    fn main() {
        let wam = 50;
        let zid = 1234567;
        let predicted_result: Option<&str> = {
            let mut result = None;
            if zid == 5205060 {
                result = Some("FL");
            }
            if wam >= 50 {
                result = Some("PS");
            }
            result
        };
    
        println!("{:?}", predicted_result);
    
    } 
}

fn foo() -> Result<String, ()> {
    // Ok(String::from("hello"))
    Err(())
    
    // todo!()
}

fn foo2() -> Option<String> {
    // Some(String::from("hello"))
    None
    
    // todo!()
}
