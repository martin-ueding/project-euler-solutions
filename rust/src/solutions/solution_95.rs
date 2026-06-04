use std::collections::HashSet;

use indicatif::ProgressIterator;

use crate::primes::{PrimeList, get_divisors};

fn sum_proper_divisors(number: i64, prime_list: &mut PrimeList) -> i64 {
    get_divisors(number, prime_list)
        .into_iter()
        .take_while(|&d| d < number)
        .sum()
}

fn iter_chain(
    start: i64,
    eventually_zero: &mut HashSet<i64>,
    prime_list: &mut PrimeList,
) -> Option<(i64, i64)> {
    let mut explored: HashSet<i64> = HashSet::new();
    let mut number = start;
    for _ in 0..1000 {
        explored.insert(number);
        number = sum_proper_divisors(number, prime_list);
        if number == 0 {
            break;
        }
        if number == start {
            break;
        }
        if number > 1_000_000 {
            break;
        }
        if explored.contains(&number) {
            break;
        }
        if eventually_zero.contains(&number) {
            number = 0;
            break;
        }
    }
    if number == 0 {
        eventually_zero.extend(explored.into_iter());
    } else if number == start {
        let chain_length = explored.len() as i64;
        let smallest_member = *explored.iter().min().unwrap();
        println!(
            "n = {start}: chain with {chain_length} elements and {smallest_member} as minimum"
        );
        return Some((chain_length, smallest_member));
    }
    return None;
}

fn solution() -> i64 {
    let mut prime_list = PrimeList::new();
    let mut eventually_zero: HashSet<i64> = HashSet::new();

    (2..1_000_001)
        .progress_count(999_999)
        .map(|n| iter_chain(n, &mut eventually_zero, &mut prime_list))
        .flatten()
        .max()
        .unwrap()
        .1
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 95,
        implementations: &[("", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_proper_divisors() {
        let mut prime_list = PrimeList::new();
        assert_eq!(sum_proper_divisors(28, &mut prime_list), 28);
        assert_eq!(sum_proper_divisors(220, &mut prime_list), 284);
        assert_eq!(sum_proper_divisors(284, &mut prime_list), 220);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 14_316);
    }
}
