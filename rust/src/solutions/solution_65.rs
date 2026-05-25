use crate::{fractions::convergent_from_continued_fraction, specnum::digit_sum};

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
    digit_sum(convergent_from_continued_fraction(&coefficients).0)
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
