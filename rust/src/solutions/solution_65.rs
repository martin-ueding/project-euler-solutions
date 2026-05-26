use crate::{fractions::convergent_from_continued_fraction_bigint, specnum::digit_sum_bigint};

struct ContinuedFractionE {
    k: i64,
    step: u8,
}

impl ContinuedFractionE {
    fn new() -> Self {
        Self { k: 1, step: 0 }
    }
}

impl Iterator for ContinuedFractionE {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.step {
            0 => 2,
            1 => 1,
            2 => 2 * self.k,
            _ => {
                self.k += 1;
                1
            }
        };

        self.step += 1;
        if self.step > 3 {
            self.step = 1;
        }

        Some(result)
    }
}

fn convergent_numerator_digit_sum(nth: i64) -> i64 {
    let coefficients: Vec<i64> = ContinuedFractionE::new().take(nth as usize).collect();
    digit_sum_bigint(convergent_from_continued_fraction_bigint(&coefficients).0)
}

fn solution() -> i64 {
    convergent_numerator_digit_sum(100)
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 65,
        implementations: &[("", solution)],
        solution: Some(272),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continued_fraction_e() {
        let expected = vec![2, 1, 2, 1, 1, 4, 1, 1, 6, 1];
        let actual: Vec<i64> = ContinuedFractionE::new().take(expected.len()).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_convergent_numerator_digit_sum() {
        assert_eq!(convergent_numerator_digit_sum(10), 17);
    }
}
