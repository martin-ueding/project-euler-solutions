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
    eventually_zero: &mut Vec<bool>,
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
        if eventually_zero[number as usize] {
            number = 0;
            break;
        }
    }
    if number == 0 {
        explored
            .into_iter()
            .for_each(|e| eventually_zero[e as usize] = true);
    } else if number == start {
        let chain_length = explored.len() as i64;
        let smallest_member = *explored.iter().min().unwrap();
        // println!(
        //     "n = {start}: chain with {chain_length} elements and {smallest_member} as minimum"
        // );
        return Some((chain_length, smallest_member));
    }
    return None;
}

fn solution() -> i64 {
    let mut prime_list = PrimeList::new();
    let mut eventually_zero: Vec<bool> = vec![false; 1_000_000];

    (2..1_000_001)
        .progress_count(999_999)
        .map(|n| iter_chain(n, &mut eventually_zero, &mut prime_list))
        .flatten()
        .max()
        .unwrap()
        .1
}

fn divisor_sums() -> Vec<i64> {
    let mut result: Vec<i64> = vec![0; 1_000_001];
    for divisor in 1..1_000_001i64 {
        for multiplier in 2.. {
            let index = divisor * multiplier;
            if index >= result.len() as i64 {
                break;
            }
            result[index as usize] += divisor;
        }
    }
    result
}

fn get_chain_length(start: i64, next: &mut Vec<i64>) -> Option<(i64, i64)> {
    // println!("start = {start}, next[12496] = {}", next[12496]);
    let mut explored: HashSet<i64> = HashSet::new();
    let mut number = start;
    loop {
        if start == 12496 {
            // println!("explored = {:?}, number = {number}", explored);
        }
        explored.insert(number);
        number = next[number as usize];
        if start == 12496 {
            // println!("  explored = {:?}, number = {number}", explored);
        }
        if number == 0 || number > 1_000_000 || explored.contains(&number) {
            break;
        }
    }
    if start == 12496 {
        // println!("explored = {:?}", explored);
    }
    if number == start {
        let chain_length = explored.len() as i64;
        let min_element = *explored.iter().min().unwrap();
        // println!("start = {start}, length = {chain_length}, min = {min_element}, {explored:?}");
        return Some((chain_length, min_element));
    } else if number == 0 {
        explored.into_iter().for_each(|n| next[n as usize] = 0);
    }
    None
}

fn solution_sieve() -> i64 {
    let mut next = divisor_sums();
    // println!("{:?}", &next[0..100]);
    (1..1_000_001)
        .map(|start| get_chain_length(start, &mut next))
        .flatten()
        .max()
        .unwrap()
        .1
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 95,
        implementations: &[("", solution_sieve)],
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
