//! Working with numbers in their (usually base-10) representation.

use itertools::Itertools;
use num_bigint::BigInt;
use num_traits::{Pow, ToPrimitive, Zero};

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

pub fn int_from_digits(digits: &[i64]) -> i64 {
    let mut result: i64 = 0;
    for digit in digits {
        result *= 10;
        result += digit;
    }
    result
}

pub fn last_9_digits_pandigital(mut number: i64) -> bool {
    if number < 100_000_000 {
        return false;
    }
    let mut mask: u32 = 0;
    for _ in 0..9 {
        let last_digit = number % 10;
        mask |= 1 << last_digit;
        number /= 10;
    }
    mask & 0b11_1111_1110 == 0b11_1111_1110
}

pub fn first_9_digits_pandigital(mut number: BigInt) -> bool {
    if number < BigInt::from(100_000_000) {
        return false;
    }
    let mut digits: Vec<i32> = Vec::new();
    while number > BigInt::zero() {
        digits.push((&number % 10i32).to_i32().unwrap());
        number /= 10;
    }
    let len = digits.len();
    let first_digits = &mut digits[len - 9..];
    first_digits.sort();
    first_digits.len() == 9 && first_digits.iter().all_unique() && first_digits[0] == 1
}

pub fn first_9_digits_pandigital_log_mantissa(log_number: f64) -> bool {
    let s = format!("{}", 10_f64.pow(log_number + 15_f64));
    let mut first_digits: Vec<char> = s.chars().take(9).collect();
    // println!("{first_digits:?}");
    first_digits.sort();
    first_digits.len() == 9 && first_digits.iter().all_unique() && first_digits[0] == '1'
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

    #[test]
    fn bit_shift() {
        assert_eq!(1 << 1, 0b10);
    }

    #[test]
    fn last_9_digits_pandigital_detects_pandigital() {
        assert!(last_9_digits_pandigital(543123456789));
    }

    #[test]
    fn last_9_digits_pandigital_detects_pandigital_2() {
        assert!(last_9_digits_pandigital(6941749852924781635));
    }

    #[test]
    fn last_9_digits_pandigital_rejects_non_pandigital() {
        assert!(!last_9_digits_pandigital(5431234564789));
    }

    #[test]
    fn first_9_digits_pandigital_detects_pandigital() {
        assert!(first_9_digits_pandigital(BigInt::from(123456789000_i64)));
    }

    #[test]
    fn first_9_digits_pandigital_rejects_non_pandigital() {
        assert!(!first_9_digits_pandigital(BigInt::from(1234056789000_i64)));
    }
}
