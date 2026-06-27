use itertools::Itertools;

fn solution() -> i64 {
    let o = optimal_special_set(7);
    set_string(&o)
}

fn optimal_special_set(n: i32) -> Vec<i32> {
    if n == 1 {
        vec![1]
    } else {
        let mut solution = vec![1, 2];
        for k in 3..n + 1 {
            let candidate = get_next_candidate(&solution);
            let sum_ceiling: i32 = candidate.iter().sum();
            solution = f(k, sum_ceiling);
        }
        solution
    }
}

fn get_next_candidate(a: &[i32]) -> Vec<i32> {
    let middle_element = a[(a.len() + 1) / 2];
    let mut result = vec![middle_element];
    result.extend(a.iter().map(|&i| i + middle_element));
    result
}

fn f(n: i32, sum_ceiling: i32) -> Vec<i32> {
    let mut best_set: Option<Vec<i32>> = None;
    let mut best_sum: Option<i32> = None;
    for a1 in 1.. {
        if a1 * n > sum_ceiling {
            break;
        }
        for a2 in a1 + 1.. {
            if a1 + a2 * (n - 1) > sum_ceiling {
                break;
            }
            let mut a: Vec<i32> = vec![a1, a2];
            // println!("{a:?}");
            if let Some(best) = g(&mut a, n, sum_ceiling) {
                let sum: i32 = best.iter().sum();
                if best_sum.is_none() || sum < best_sum.unwrap() {
                    best_set = Some(best);
                    best_sum = Some(sum);
                }
            }
        }
    }
    best_set.unwrap()
}

fn g(a: &mut Vec<i32>, n: i32, sum_ceiling: i32) -> Option<Vec<i32>> {
    // for _ in 0..a.len() {
    //     print!(" ");
    // }
    // println!("{a:?}");
    let mut best_set: Option<Vec<i32>> = None;
    let mut best_sum: Option<i32> = None;
    if is_special_sum_set(a) {
        // println!("{a:?} special");
        if a.len() < (n as usize) {
            for possible_number in get_possible_numbers(&a) {
                if a.iter().sum::<i32>() + possible_number > sum_ceiling {
                    break;
                }
                a.push(possible_number);
                let best_result = g(a, n, sum_ceiling);
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

fn is_special_sum_set(a: &[i32]) -> bool {
    satisfies_larger_constraint(a)
        && a.iter()
            .copied()
            .permutations(a.len())
            .all(|p| is_valid_permutation(&p))
}

/// Checks for all B, C: |B| > |C| => S(B) > S(C).
fn satisfies_larger_constraint(a: &[i32]) -> bool {
    (1..(a.len() + 1) / 2)
        .all(|k| a[..k + 1].iter().sum::<i32>() >= a[a.len() - k..].iter().sum::<i32>())
}

fn is_valid_permutation(a: &[i32]) -> bool {
    for m in 1..a.len() - 1 {
        for n in 1..a.len() - m + 1 {
            let b = &a[..m];
            let c = &a[m..m + n];
            if !is_unequal_subsets(&b, &c) {
                return false;
            }
        }
    }
    true
}

/// Checks sum(B) != sum(C).
fn is_unequal_subsets(b: &[i32], c: &[i32]) -> bool {
    b.iter().sum::<i32>() != c.iter().sum::<i32>()
}

fn get_possible_numbers(a: &[i32]) -> Vec<i32> {
    let begin = *a.last().unwrap() + 1;
    let end: i32 = a.iter().sum();
    let mut disallowed: Vec<i32> = (1..a.len() + 1)
        .map(|k| {
            a.iter()
                .permutations(k)
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
    fn optimal_special_set_n1() {
        assert_eq!(optimal_special_set(1), vec![1]);
    }

    #[test]
    fn optimal_special_set_n2() {
        assert_eq!(optimal_special_set(2), vec![1, 2]);
    }

    #[test]
    fn optimal_special_set_n3() {
        assert_eq!(optimal_special_set(3), vec![2, 3, 4]);
    }

    #[test]
    fn optimal_special_set_n4() {
        assert_eq!(optimal_special_set(4), vec![3, 5, 6, 7]);
    }

    #[test]
    fn optimal_special_set_n5() {
        assert_eq!(optimal_special_set(5), vec![6, 9, 11, 12, 13]);
    }

    #[test]
    fn optimal_special_set_n6() {
        assert_eq!(optimal_special_set(6), vec![11, 18, 19, 20, 22, 25]);
    }

    #[test]
    fn is_special_sum_set_accepts_n5_example() {
        assert!(is_special_sum_set(&vec![6, 9, 11, 12, 13]));
    }
    #[test]
    fn is_special_sum_set_rejects_consecutive() {
        assert!(!is_special_sum_set(&vec![4, 5, 6, 7]));
    }

    #[test]
    fn satisfies_larger_constraint_accepts_n4_solution() {
        assert!(satisfies_larger_constraint(&vec![3, 5, 6, 7]));
    }

    #[test]
    fn satisfies_larger_constraint_accepts_n5_solution() {
        assert!(satisfies_larger_constraint(&vec![6, 9, 11, 12, 13]));
    }

    #[test]
    fn satisfies_larger_constraint_rejects_invalid_set() {
        assert!(!satisfies_larger_constraint(&vec![1, 2, 3, 5]));
    }

    #[test]
    fn is_unequal_subsets_accepts_unequal_sums() {
        assert!(is_unequal_subsets(&vec![1, 2], &vec![4, 5]));
    }

    #[test]
    fn is_unequal_subsets_rejects_equal_sums() {
        assert!(!is_unequal_subsets(&vec![1, 4], &vec![2, 3]));
    }

    #[test]
    fn get_possible_numbers_1() {
        assert_eq!(get_possible_numbers(&vec![4, 5]), vec![6, 7, 8])
    }

    #[test]
    fn set_string_for_n6() {
        assert_eq!(set_string(&vec![11, 18, 19, 20, 22, 25]), 111819202225);
    }

    #[test]
    fn solution_matches() {
        assert_eq!(solution(), 20_313_839_404_245);
    }

    #[test]
    fn get_next_candidate_constructs_4_to_5() {
        assert_eq!(
            get_next_candidate(&vec![3, 5, 6, 7]),
            vec![6, 9, 11, 12, 13]
        );
    }
}
