use crate::diophantine::DiophantineSolutionIterator;

fn solution() -> i64 {
    let mut result = 0;
    for (x, y) in DiophantineSolutionIterator::new(3, 4) {
        let a_factor = 3;

        // The case (a, a, a+1).
        if (x + 1) % a_factor == 0 {
            let a = (x + 1) / a_factor;
            if (y * (a + 1)) % 4 != 0 {
                continue;
            }
            let p = 3 * a + 1;
            if p > 1_000_000_000 {
                break;
            }
            result += p;
        }
        // The case (a, a, a-1).
        else if (x - 1) % a_factor == 0 {
            let a = (x - 1) / a_factor;
            if (y * (a - 1)) % 4 != 0 {
                continue;
            }
            let p = 3 * a - 1;
            if p > 1_000_000_000 {
                break;
            }
            if p > 4 {
                result += p;
            }
        }
    }
    result
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 94,
        implementations: &[("diophantine solutions", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 518_408_346);
    }
}
