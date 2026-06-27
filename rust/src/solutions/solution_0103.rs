fn optimal_special_set(n: i64) -> Vec<i64> {
    vec![]
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
}
