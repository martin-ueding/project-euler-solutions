use std::collections::HashSet;

fn possible_products(n: i64, k: i64) -> HashSet<i64> {
    if k == 1 {
        HashSet::from([n])
    } else {
        let mut result: HashSet<i64> = HashSet::new();
        for i in 1..n - k + 2 {
            possible_products(n - i, k - 1).iter().for_each(|p| {
                println!("n={n}, k={k}, i={i}, p={p}");
                result.insert(i * p);
            });
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_products() {
        assert_eq!(possible_products(1, 1), HashSet::from([1]));
        assert_eq!(possible_products(2, 1), HashSet::from([2]));
        assert_eq!(possible_products(2, 2), HashSet::from([1]));
        assert_eq!(possible_products(3, 2), HashSet::from([2]));
        assert_eq!(possible_products(4, 2), HashSet::from([3, 4]));
    }
}
