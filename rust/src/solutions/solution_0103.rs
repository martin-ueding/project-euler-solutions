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
}
