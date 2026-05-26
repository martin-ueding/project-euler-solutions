use num_bigint::BigInt;
use num_traits::ToPrimitive;

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

pub fn digit_sum(mut number: i64) -> i64 {
    let mut result = 0;
    while number > 0 {
        result += number % 10;
        number /= 10;
    }
    result
}

pub fn digit_sum_bigint(mut number: BigInt) -> i64 {
    let mut result = 0;
    while number > BigInt::ZERO {
        result = result + (&number % BigInt::from(10)).to_i64().unwrap_or(0);
        number /= 10;
    }
    result
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

    #[test]
    fn test_digit_sum() {
        assert_eq!(digit_sum(1457), 17);
    }

    #[test]
    fn test_digit_sum_bigint() {
        assert_eq!(digit_sum_bigint(BigInt::from(1457)), 17);
    }
}
