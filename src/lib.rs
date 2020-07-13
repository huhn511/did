pub mod utils;
pub mod DID;


pub fn greet() -> String {
    "Hello World!".to_owned()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!("Hello World!", greet())
    }
}