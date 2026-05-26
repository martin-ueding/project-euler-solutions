use std::collections::HashMap;

fn compute_length(start: i64, mut cache: &mut HashMap<i64, i64>) -> i64 {
    if let Some(&result) = cache.get(&start) {
        return result;
    } else {
        let result = match start {
            1 => 0,
            _ => {
                1 + if start % 2 == 0 {
                    compute_length(start / 2, &mut cache)
                } else {
                    compute_length(3 * start + 1, &mut cache)
                }
            }
        };
        cache.insert(start, result);
        return result;
    }
}

fn solution_14_cache() -> i64 {
    let mut cache: HashMap<i64, i64> = HashMap::new();
    (1..1_000_000)
        .map(|start| (compute_length(start, &mut cache), start))
        .max()
        .unwrap()
        .1
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 14,
        implementations: &[("cache", solution_14_cache)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_14_cache() {
        assert_eq!(solution_14_cache(), 837_799);
    }
}
