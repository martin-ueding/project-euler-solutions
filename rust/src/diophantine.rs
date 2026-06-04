use crate::primes::{PrimeList, is_square};

/// Verifies a solution for $x^2 - D y^2 = c$.
pub fn solves_diophantine_equation(x: i64, y: i64, d: i64, c: i64) -> bool {
    x * x - d * y * y == c
}

pub fn find_initial_solution(d: i64, c: i64, prime_generator: &mut PrimeList) -> (i64, i64) {
    for y in 0.. {
        let x_sq = d * y * y + c;
        if is_square(x_sq, prime_generator) {
            let x = x_sq.isqrt();
            return (x, y);
        }
    }
    panic!("Couldn't find a solution!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solves_diophantine_equation() {
        assert!(solves_diophantine_equation(2, 0, 12, 4));
    }

    #[test]
    fn test_find_initial_solution() {
        let mut prime_generator = PrimeList::new();

        assert_eq!(find_initial_solution(12, 4, &mut prime_generator), (2, 0));
    }
}
