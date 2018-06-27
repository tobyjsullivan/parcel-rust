#[no_mangle]
pub fn say_hello() -> String {
    "Hello, world!".to_string()
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let sum = add(3, 5);
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_say_hello() {
        let out = say_hello();
        assert_eq!(out, "Hello, world!");
    }
}
