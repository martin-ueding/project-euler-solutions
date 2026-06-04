use crate::primes::{PrimeList, is_square};

/// Verifies a solution for $x^2 - D y^2 = c$.
pub fn solves_diophantine_equation(x: i64, y: i64, d: i64, c: i64) -> bool {
    x * x - d * y * y == c
}

/// Find the minimal solution for $x^2 - D y^2 = c$.
pub fn find_initial_solution(d: i64, c: i64, prime_generator: &mut PrimeList) -> (i64, i64) {
    for y in 1.. {
        let x_sq = d * y * y + c;
        if is_square(x_sq, prime_generator) {
            let x = x_sq.isqrt();
            return (x, y);
        }
    }
    panic!("Couldn't find a solution!")
}

pub struct DiophantineSolutionIterator {
    d: i64,
    x: i64,
    y: i64,
    x_hat: i64,
    y_hat: i64,
}

impl DiophantineSolutionIterator {
    pub fn new(d: i64, c: i64, prime_generator: &mut PrimeList) -> Self {
        let (x, y) = find_initial_solution(d, c, prime_generator);
        let (x_hat, y_hat) = find_initial_solution(d, 1, prime_generator);
        DiophantineSolutionIterator {
            d,
            x,
            y,
            x_hat,
            y_hat,
        }
    }
}

impl Iterator for DiophantineSolutionIterator {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let result: Self::Item = (self.x, self.y);

        let x_next = self.x_hat * self.x + self.d * self.y_hat * self.y;
        let y_next = self.y_hat * self.x + self.x_hat * self.y;
        self.x = x_next;
        self.y = y_next;

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solves_diophantine_equation() {
        assert!(solves_diophantine_equation(2, 0, 12, 4));
        assert!(!solves_diophantine_equation(2, 1, 12, 4));
    }

    #[test]
    fn test_find_initial_solution() {
        let mut prime_generator = PrimeList::new();

        assert_eq!(find_initial_solution(12, 4, &mut prime_generator), (4, 1));
    }

    #[test]
    fn test_diophantine_solution_iterator() {
        let d = 12;
        let c = 4;
        let mut prime_generator = PrimeList::new();

        let solution_iterator = DiophantineSolutionIterator::new(d, c, &mut prime_generator);
        assert!(
            solution_iterator
                .take(5)
                .all(|(x, y)| solves_diophantine_equation(x, y, d, c))
        );
    }
}
