use num_bigint::BigInt;
use num_traits::Pow;
use num_traits::{One, Zero};
pub struct FibonacciIterator {
    a: i64,
    b: i64,
}

impl FibonacciIterator {
    pub fn new() -> Self {
        FibonacciIterator { a: 0, b: 1 }
    }
}

impl Iterator for FibonacciIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.b;
        let c = self.a + self.b;
        self.a = self.b;
        self.b = c;
        Some(result)
    }
}

pub struct FibonacciSuffixIterator {
    a: i64,
    b: i64,
}

impl FibonacciSuffixIterator {
    pub fn new() -> Self {
        FibonacciSuffixIterator { a: 0, b: 1 }
    }
}

impl Iterator for FibonacciSuffixIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.b;
        let c = (self.a + self.b) % 1_000_000_000;
        self.a = self.b;
        self.b = c;
        Some(result)
    }
}

pub struct BigFibonacciIterator {
    a: BigInt,
    b: BigInt,
}

impl BigFibonacciIterator {
    pub fn new() -> Self {
        BigFibonacciIterator {
            a: BigInt::zero(),
            b: BigInt::one(),
        }
    }
}

impl Iterator for BigFibonacciIterator {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let c = &self.a + &self.b;
        self.a = std::mem::replace(&mut self.b, c);
        Some(self.a.clone())
    }
}

struct BigMatrix22 {
    a: BigInt,
    b: BigInt,
    c: BigInt,
    d: BigInt,
}

impl Clone for BigMatrix22 {
    fn clone(&self) -> Self {
        BigMatrix22 {
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
            d: self.d.clone(),
        }
    }
}

impl BigMatrix22 {
    fn multiply(&self, other: &BigMatrix22) -> BigMatrix22 {
        BigMatrix22 {
            a: &self.a * &other.a + &self.b * &other.c,
            b: &self.a * &other.b + &self.b * &other.d,
            c: &self.c * &other.a + &self.d * &other.c,
            d: &self.c * &other.b + &self.d * &other.d,
        }
    }

    fn square(&self) -> BigMatrix22 {
        self.multiply(self)
    }

    fn pow(&self, n: i64) -> BigMatrix22 {
        if n == 1 {
            self.clone()
        } else if n % 2 == 0 {
            self.square().pow(n / 2)
        } else {
            self.multiply(&self.square().pow((n - 1) / 2))
        }
    }
}

pub fn direct_fibonacci(n: i64) -> BigInt {
    let m = BigMatrix22 {
        a: BigInt::zero(),
        b: BigInt::one(),
        c: BigInt::one(),
        d: BigInt::one(),
    };
    m.pow(n).b
}

pub fn approximate_fibonacci(n: i64) -> f64 {
    println!("{n}");
    let alpha: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let beta: f64 = (1.0 - 5.0_f64.sqrt()) / 2.0;
    let result = -beta / 5.0_f64.sqrt() * (alpha.pow((n + 1) as f64) + beta.pow((n + 1) as f64));
    assert!(result.is_finite());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_iterator_first_10() {
        assert_eq!(
            FibonacciIterator::new().take(10).collect::<Vec<i64>>(),
            vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
        );
    }

    #[test]
    fn fibonacci_suffix_iterator_first_10() {
        assert_eq!(
            FibonacciSuffixIterator::new()
                .take(10)
                .collect::<Vec<i64>>(),
            vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
        );
    }

    #[test]
    fn big_fibonacci_iterator_first_10() {
        assert_eq!(
            BigFibonacciIterator::new()
                .take(10)
                .collect::<Vec<BigInt>>(),
            vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
                .into_iter()
                .map(BigInt::from)
                .collect::<Vec<BigInt>>()
        );
    }

    #[test]
    fn direct_fibonacci_computes_f100() {
        assert_eq!(
            direct_fibonacci(100),
            BigInt::from(354224848179261915075_i128)
        );
    }
}
