#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // All good
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        // we give the assert! macro an argument that evaluates to a bool. If the value is true,
        // assert! does nothing and the test passes. If the value is false, the assert!
        // macro calls the panic!
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
        assert_ne!(5, add_two(4));
    }

    #[test]
    fn it_adds_one_hundred() {
        assert_eq!(110, add_one_hundred(10));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // You can also add a custom message to be printed with the failure message as optional
        // arguments to the assert!, assert_eq!, and assert_ne! macros. Any arguments specified
        // after the one required argument to assert! or the two required arguments to assert_eq!
        // and assert_ne! are passed along to the format! macro, so you can pass a format string
        // that contains {} placeholders and values to go in those placeholders.
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {}",
            result
        );
    }

    #[test]
    // This attribute makes a test pass if the code inside the function panics;
    // the test will fail if the code inside the function doesn’t panic.
    #[should_panic(expected = "Guess value must be equal to or less than 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(5);
    //     assert_eq!(5, value);
    // }

    fn internal() {
        // Test a private function
        assert_eq!(4, internal_adder(2, 2));
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_one_hundred(a: i32) -> i32 {
    a + 100
}

pub fn add_two_again(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be equal to or greater than 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be equal to or less than 100, got {}.", value);
        }

        Guess { value }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
