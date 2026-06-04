use crate::primes::{PrimeList, get_divisors};

fn sum_proper_divisors(number: i64, prime_list: &mut PrimeList) -> i64 {
    get_divisors(number, prime_list)
        .into_iter()
        .take_while(|&d| d < number)
        .sum()
}

// inventory::submit! {
//     crate::registry::SolutionEntry {
//         id: 95,
//         implementations: &[("", solution)],
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut prime_list = PrimeList::new();
        assert_eq!(sum_proper_divisors(28, &mut prime_list), 28);
        assert_eq!(sum_proper_divisors(220, &mut prime_list), 284);
        assert_eq!(sum_proper_divisors(284, &mut prime_list), 220);
    }
}
