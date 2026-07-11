use itertools::Itertools;

use crate::special_sum_sets::is_special_sum_set;

fn solution() -> i64 {
    let o = find_optimal_special_set(7);
    set_string(&o)
}

fn find_optimal_special_set(n: i32) -> Vec<i32> {
    if n == 1 {
        vec![1]
    } else {
        let mut solution = vec![1, 2];
        for k in 3..n + 1 {
            let candidate = get_next_candidate(&solution);
            let sum_ceiling: i32 = candidate.iter().sum();
            solution = find_osss(k, sum_ceiling);
        }
        solution
    }
}

/// Apply {a_1, a_2, …, a_n} -> {b, a_1 + b, a_2 + b, …}.
fn get_next_candidate(a: &[i32]) -> Vec<i32> {
    let middle_element = a[(a.len() + 1) / 2];
    let mut result = vec![middle_element];
    result.extend(a.iter().map(|&i| i + middle_element));
    result
}

fn find_osss(n: i32, sum_ceiling: i32) -> Vec<i32> {
    (1..)
        .take_while(|&a1| a1 * n <= sum_ceiling)
        .flat_map(|a1| {
            (a1 + 1..)
                .take_while(move |a2| a1 + a2 * (n - 1) <= sum_ceiling)
                .map(move |a2| (a1, a2))
        })
        .map(|(a1, a2)| find_osss_from_prefix(a1, a2, n, sum_ceiling))
        .flatten()
        .min_by_key(|a| a.iter().sum::<i32>())
        .unwrap()
}

fn find_osss_from_prefix(a1: i32, a2: i32, n: i32, sum_ceiling: i32) -> Option<Vec<i32>> {
    let mut a: Vec<i32> = vec![a1, a2];
    complete_sss(&mut a, n, sum_ceiling)
}

fn complete_sss(a: &mut Vec<i32>, n: i32, sum_ceiling: i32) -> Option<Vec<i32>> {
    let mut best_set: Option<Vec<i32>> = None;
    let mut best_sum: Option<i32> = None;
    if is_special_sum_set(a) {
        if a.len() < (n as usize) {
            for possible_number in get_possible_numbers(&a) {
                if a.iter().sum::<i32>() + possible_number > sum_ceiling {
                    break;
                }
                a.push(possible_number);
                let best_result = complete_sss(a, n, sum_ceiling);
                if best_result.is_some() {
                    let sum: i32 = best_result.as_ref().unwrap().iter().sum();
                    if best_sum.is_none() || sum < best_sum.unwrap() {
                        best_set = best_result;
                        best_sum = Some(sum);
                    }
                }
                a.pop();
            }
        } else {
            best_set = Some(a.clone());
        }
    }
    best_set
}

/// Find numbers that can be appended to the given set.
fn get_possible_numbers(a: &[i32]) -> Vec<i32> {
    let begin = *a.last().unwrap() + 1;
    let end: i32 = a.iter().sum();
    let mut disallowed: Vec<i32> = (1..a.len() + 1)
        .map(|k| {
            a.iter()
                .combinations(k)
                .map(|permutation| permutation.into_iter().copied().sum())
        })
        .flatten()
        .sorted()
        .dedup()
        .collect();
    disallowed.reverse();
    let mut allowed: Vec<i32> = Vec::new();
    for i in begin..end {
        if let Some(&d) = disallowed.last() {
            if i == d {
                disallowed.pop();
                continue;
            }
        }
        allowed.push(i);
    }
    allowed
}

/// Convert set into digit concatenation.
fn set_string(a: &[i32]) -> i64 {
    let s: String = a.iter().map(|&i| i.to_string()).collect();
    s.parse().unwrap()
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 103,
        implementations: &[("", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_matches() {
        assert_eq!(solution(), 20_313_839_404_245);
    }

    #[test]
    fn optimal_special_set_n1() {
        assert_eq!(find_optimal_special_set(1), vec![1]);
    }

    #[test]
    fn optimal_special_set_n2() {
        assert_eq!(find_optimal_special_set(2), vec![1, 2]);
    }

    #[test]
    fn optimal_special_set_n3() {
        assert_eq!(find_optimal_special_set(3), vec![2, 3, 4]);
    }

    #[test]
    fn optimal_special_set_n4() {
        assert_eq!(find_optimal_special_set(4), vec![3, 5, 6, 7]);
    }

    #[test]
    fn optimal_special_set_n5() {
        assert_eq!(find_optimal_special_set(5), vec![6, 9, 11, 12, 13]);
    }

    #[test]
    fn optimal_special_set_n6() {
        assert_eq!(find_optimal_special_set(6), vec![11, 18, 19, 20, 22, 25]);
    }

    #[test]
    fn get_next_candidate_constructs_4_to_5() {
        assert_eq!(
            get_next_candidate(&vec![3, 5, 6, 7]),
            vec![6, 9, 11, 12, 13]
        );
    }

    #[test]
    fn get_possible_numbers_1() {
        assert_eq!(get_possible_numbers(&vec![4, 5]), vec![6, 7, 8])
    }

    #[test]
    fn set_string_for_n6() {
        assert_eq!(set_string(&vec![11, 18, 19, 20, 22, 25]), 111819202225);
    }
}
