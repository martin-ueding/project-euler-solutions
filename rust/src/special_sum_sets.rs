use itertools::Itertools;

pub fn special_sum_set_from_conway_guy(k: i64) -> Vec<i64> {
    (1..k + 1)
        .map(|i| conway_guy_series(k) - conway_guy_series(k - i))
        .collect()
}

fn conway_guy_series(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        n => {
            2 * conway_guy_series(n - 1)
                - conway_guy_series(n - 1 - (0.5 + ((2 * (n - 1)) as f64).sqrt()).floor() as i64)
        }
    }
}

pub fn is_special_sum_set(a: &[i32]) -> bool {
    is_size_monotone(a)
        && a.iter()
            .copied()
            .permutations(a.len())
            .all(|p| is_sum_distinct(&p))
}

/// Checks for all B, C: |B| > |C| => S(B) > S(C).
fn is_size_monotone(a: &[i32]) -> bool {
    (1..(a.len() + 1) / 2)
        .all(|k| a[..k + 1].iter().sum::<i32>() >= a[a.len() - k..].iter().sum::<i32>())
}

/// Verifies all partitions in this permutation.
fn is_sum_distinct(a: &[i32]) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_sum_set_from_conway_guy_k_5() {
        assert_eq!(special_sum_set_from_conway_guy(5), vec![6, 9, 11, 12, 13]);
    }

    #[test]
    fn special_sum_set_from_conway_guy_k_6() {
        assert_eq!(
            special_sum_set_from_conway_guy(6),
            vec![11, 17, 20, 22, 23, 24]
        );
    }

    #[test]
    fn conway_guy_series_elements() {
        let actual: Vec<i64> = (0..12).map(conway_guy_series).collect();
        let expected = vec![0, 1, 2, 4, 7, 13, 24, 44, 84, 161, 309, 594];
        assert_eq!(actual, expected);
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
    fn is_size_monotone_accepts_n4_solution() {
        assert!(is_size_monotone(&vec![3, 5, 6, 7]));
    }

    #[test]
    fn is_size_monotone_accepts_n5_solution() {
        assert!(is_size_monotone(&vec![6, 9, 11, 12, 13]));
    }

    #[test]
    fn is_size_monotone_rejects_invalid_set() {
        assert!(!is_size_monotone(&vec![1, 2, 3, 5]));
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
