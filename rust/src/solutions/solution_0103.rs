use itertools::Itertools;

fn optimal_special_set(n: i64) -> Vec<i64> {
    vec![]
}

fn is_unequal_subsets(b: &[i64], c: &[i64]) -> bool {
    b.iter().sum::<i64>() != c.iter().sum::<i64>()
}

fn satisfies_larger_constraint(a: &[i64]) -> bool {
    (1..(a.len() + 1) / 2)
        .all(|k| a[..k + 1].iter().sum::<i64>() >= a[a.len() - k..].iter().sum::<i64>())
}

fn is_valid_permutation(a: &[i64]) -> bool {
    for m in 1..a.len() - 1 {
        for n in 1..a.len() - m {
            let b = &a[..m];
            let c = &a[m..m + n];
            if !is_unequal_subsets(&b, &c) {
                return false;
            }
        }
    }
    true
}

fn is_special_sum_set(a: &[i64]) -> bool {
    satisfies_larger_constraint(a)
        && a.iter()
            .copied()
            .permutations(a.len())
            .all(|p| is_valid_permutation(&p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimal_special_set_1() {
        assert_eq!(optimal_special_set(1), vec![1]);
    }

    #[test]
    fn test_optimal_special_set_2() {
        assert_eq!(optimal_special_set(2), vec![1, 2]);
    }

    #[test]
    fn test_optimal_special_set_3() {
        assert_eq!(optimal_special_set(3), vec![2, 3, 4]);
    }

    #[test]
    fn test_optimal_special_set_4() {
        assert_eq!(optimal_special_set(4), vec![3, 5, 6, 7]);
    }

    #[test]
    fn test_optimal_special_set_5() {
        assert_eq!(optimal_special_set(5), vec![6, 9, 11, 12, 13]);
    }

    #[test]
    fn test_optimal_special_set_6() {
        assert_eq!(optimal_special_set(6), vec![11, 18, 19, 20, 22, 25]);
    }

    #[test]
    fn test_is_special_sum_set_example_6() {
        assert!(is_special_sum_set(&vec![6, 9, 11, 12, 13]));
    }

    #[test]
    fn test_satisfies_larger_constraint_1() {
        assert!(satisfies_larger_constraint(&vec![3, 5, 6, 7]));
    }

    #[test]
    fn test_satisfies_larger_constraint_2() {
        assert!(satisfies_larger_constraint(&vec![6, 9, 11, 12, 13]));
    }

    #[test]
    fn test_satisfies_larger_constraint_3() {
        assert!(!satisfies_larger_constraint(&vec![1, 2, 3, 5]));
    }

    #[test]
    fn test_is_unequal_subsets_true() {
        assert!(is_unequal_subsets(&vec![1, 2], &vec![4, 5]));
    }

    #[test]
    fn test_is_unequal_subsets_false() {
        assert!(!is_unequal_subsets(&vec![1, 4], &vec![2, 3]));
    }
}
