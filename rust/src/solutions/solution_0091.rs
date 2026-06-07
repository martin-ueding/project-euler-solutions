fn is_right_triangle(x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
    let mut sides = vec![
        x1.pow(2) + y1.pow(2),
        x2.pow(2) + y2.pow(2),
        (x1 - x2).pow(2) + (y1 - y2).pow(2),
    ];
    sides.sort();
    sides[0] > 0 && sides[0] + sides[1] == sides[2]
}

fn number_brute_force(limit: i64) -> i64 {
    let mut result = 0;
    for x1 in 0..limit + 1 {
        for x2 in x1..limit + 1 {
            for y1 in 0..limit + 1 {
                for y2 in 0..limit + 1 {
                    if (x1, y1) > (x2, y2) {
                        continue;
                    }
                    if is_right_triangle(x1, y1, x2, y2) {
                        result += 1
                    }
                }
            }
        }
    }
    result
}

fn solution_brute_force() -> i64 {
    number_brute_force(50)
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 91,
        implementations: &[("testing all combinations", solution_brute_force)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_brute_force() {
        assert_eq!(number_brute_force(2), 14);
    }

    #[test]
    fn test_solution_brute_force() {
        assert_eq!(solution_brute_force(), 14_234);
    }
}
