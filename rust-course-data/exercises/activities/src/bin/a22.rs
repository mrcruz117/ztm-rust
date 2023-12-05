// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::result;

    use crate::*;

    #[test]
    fn test_clamp_lower() {
        assert_eq!(clamp(1, 2, 3), 2);
    }

    #[test]
    fn test_clamp_upper() {
        assert_eq!(clamp(4, 2, 3), 3);
    }

    #[test]
    fn test_clamp_in_range() {
        assert_eq!(clamp(3, 2, 4), 3);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(4, 2), Some(2));
    }

    #[test]
    fn test_div_by_zero() {
        let result = div(4, 0);
        let expected = None;
        assert_eq!(result, expected, "Cannot divide by zero");
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat("Hello", "World"), "HelloWorld");
    }
}
