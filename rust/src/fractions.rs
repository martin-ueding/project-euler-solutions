use crate::integers;

pub type Fraction = (i64, i64);

/// Cancel a fraction using the GCD.
pub fn cancelled((n, d): Fraction) -> Fraction {
    let gcd = integers::greatest_common_denominator(n, d);
    (n / gcd, d / gcd)
}

/// Cancel a fraction using the GCD.
pub fn cancel(n: &mut i64, d: &mut i64) {
    let gcd = integers::greatest_common_denominator(*n, *d);
    *n /= gcd;
    *d /= gcd;
}
