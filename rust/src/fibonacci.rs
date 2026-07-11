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

pub struct OverflowingFibonacciIterator {
    a: u64,
    b: u64,
}

impl OverflowingFibonacciIterator {
    pub fn new() -> Self {
        OverflowingFibonacciIterator { a: 0, b: 1 }
    }
}

impl Iterator for OverflowingFibonacciIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.b;
        let c = self.a.wrapping_add(self.b);
        self.a = self.b;
        self.b = c;
        Some(result)
    }
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
    fn overflowing_fibonacci_iterator_first_10() {
        assert_eq!(
            OverflowingFibonacciIterator::new()
                .take(10)
                .collect::<Vec<u64>>(),
            vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
        );
    }
}
