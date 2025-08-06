// cli.rs is going to contain the `clap` logic to process command line arguments

// Doc comments like this are extremely useful for our code
pub fn hello() -> String {
    "Hello Hello Hello!".to_string()
}


// IMPORTANT: Unit tests need to be written in our files since our UI is coming last
#[cfg(test)]
mod tests {
    use crate::hello;
    
    #[test]
    fn test_hello_returns_expected_string() {
        let result = hello();
        assert_eq!(result, "Hello Hello Hello!");
    }
}