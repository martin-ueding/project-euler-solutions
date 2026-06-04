use num_bigint::BigInt;
use num_traits::One;

use crate::{continued_fractions::ConvergentSeries, integers::{is_square, is_square_bigint}};

/// Verifies a solution for $x^2 - D y^2 = c$.
pub fn solves_diophantine_equation(x: &BigInt, y: &BigInt, d: i64, c: i64) -> bool {
    x.pow(2) - d * y.pow(2) == BigInt::from(c)
}

/// Find the minimal solution for $x^2 - D y^2 = c$.
pub fn find_initial_solution(d: i64, c: i64) -> (BigInt, BigInt) {
    if c == 1 {
        return find_initial_solution_pell(d);
    }
    if is_square(d) {
        panic!("d = {d} must not be a square number!");
    }
    let mut y = BigInt::one();
    loop {
        let x_sq = d * &y * &y + c;
        if is_square_bigint(&x_sq) {
            let x = x_sq.sqrt();
            return (x, y);
        }
        y += 1;
    }
}

fn find_initial_solution_pell(d: i64) -> (BigInt, BigInt) {
    for (x, y) in ConvergentSeries::new(d) {
        if x.is_one() {
            continue;
        }
        if solves_diophantine_equation(&x, &y, d, 1) {
            return (x, y);
        }
    }
    panic!("Couldn't find a solution!");
}

pub struct DiophantineSolutionIterator {
    d: i64,
    x: BigInt,
    y: BigInt,
    x_hat: BigInt,
    y_hat: BigInt,
}

impl DiophantineSolutionIterator {
    pub fn new(d: i64, c: i64) -> Self {
        let (x, y) = find_initial_solution(d, c);
        let (x_hat, y_hat) = find_initial_solution(d, 1);
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
    type Item = (BigInt, BigInt);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.x.clone(), self.y.clone());

        let x_next = &self.x_hat * &self.x + self.d * &self.y_hat * &self.y;
        let y_next = &self.y_hat * &self.x + &self.x_hat * &self.y;
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
        assert!(solves_diophantine_equation(&BigInt::from(2), &BigInt::from(0), 12, 4));
        assert!(!solves_diophantine_equation(&BigInt::from(2), &BigInt::from(1), 12, 4));
    }

    #[test]
    fn test_find_initial_solution() {
        assert_eq!(find_initial_solution(12, 4), (BigInt::from(4), BigInt::from(1)));
    }

    #[test]
    fn test_diophantine_solution_iterator() {
        let d = 12;
        let c = 4;

        let solution_iterator = DiophantineSolutionIterator::new(d, c);
        assert!(
            solution_iterator
                .take(5)
                .all(|(x, y)| solves_diophantine_equation(&x, &y, d, c))
        );
    }
}