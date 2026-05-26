/// Compute greatest common denominator (GCD) using Euclid's algorithm.
pub fn greatest_common_denominator(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_greatest_common_denominator() {
        assert_eq!(greatest_common_denominator(1071, 462), 21);
    }
}
