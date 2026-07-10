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
}
