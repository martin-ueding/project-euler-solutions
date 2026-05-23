/// Checks whether a number is a palindrome.
pub fn is_palindrome(number: i64) -> bool {
    let mut reversed: i64 = 0;
    let mut remainder = number;
    while remainder > 0 {
        reversed *= 10;
        let last_digit = remainder % 10;
        reversed += last_digit;
        remainder /= 10;
    }
    number == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome(0), true);
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(11), true);
        assert_eq!(is_palindrome(1001), true);
        assert_eq!(is_palindrome(1002), false);
    }
}
