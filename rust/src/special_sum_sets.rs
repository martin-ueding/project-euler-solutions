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
    if is_size_monotone(a) {
        let t1 = is_sum_distinct(a);
        let t2 = is_equal_size_sum_distinct(a);
        if t1 != t2 {
            println!("Mismatch with {a:?}, {t1} vs. {t2}");
        }
        t1
    } else {
        false
    }
}

/// Checks for all B, C: |B| > |C| => S(B) > S(C).
fn is_size_monotone(a: &[i32]) -> bool {
    (1..(a.len() + 1) / 2)
        .all(|k| a[..k + 1].iter().sum::<i32>() > a[a.len() - k..].iter().sum::<i32>())
}

fn is_sum_distinct(a: &[i32]) -> bool {
    a.iter()
        .copied()
        .permutations(a.len())
        .all(|p| is_permutation_sum_distinct(&p))
}

/// Verifies all partitions in this permutation.
fn is_permutation_sum_distinct(a: &[i32]) -> bool {
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

fn is_equal_size_sum_distinct(a: &[i32]) -> bool {
    (1..a.len() / 2 + 1).all(|k| {
        a.iter().copied().combinations(k).all(|set_1| {
            let sum_1: i32 = set_1.iter().sum();
            a.iter()
                .copied()
                .filter(|elem| !set_1.contains(elem))
                .combinations(k)
                .all(|set_2| set_2.iter().sum::<i32>() != sum_1)
        })
    })
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
        assert!(!is_size_monotone(&vec![2, 4, 5, 6]));
        assert!(!is_size_monotone(&vec![2, 12, 13, 14]));
    }

    #[test]
    fn is_equal_size_sum_distinct_accepts() {
        assert!(is_equal_size_sum_distinct(&vec![6, 9, 11, 12, 13]));
    }

    #[test]
    fn is_equal_size_sum_distinct_rejects() {
        assert!(!is_equal_size_sum_distinct(&vec![1, 2, 3, 4]));
    }
}
