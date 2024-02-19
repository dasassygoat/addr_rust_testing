use std::fmt::format;

pub fn add_two(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    format!("Hello") // no semi colon
}
pub struct Guess {
    value:i32,
}
impl Guess{
    pub fn new(value:i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}",
            value);
        }

        Guess {value}
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn it_adds_two() {
    //  assert_eq!(4, add_two(2));
    //}

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(23);
    }
}
