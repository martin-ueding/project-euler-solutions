use itertools::{Itertools, repeat_n};
use num_rational::Rational32;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

use crate::digits::int_from_digits;

fn consecutive_ceiling_from_digits(digits: &[i32]) -> i32 {
    let operations: &[fn(Rational32, Rational32) -> Option<Rational32>] = &[
        |a, b| a.checked_add(&b),
        |a, b| a.checked_sub(&b),
        |a, b| a.checked_mul(&b),
        |a, b| a.checked_div(&b),
    ];

    let possible_numbers: Vec<i32> = digits
        .iter()
        .permutations(4)
        .cartesian_product(repeat_n(operations.iter(), 3).multi_cartesian_product())
        .map(|(d, o)| {
            let d2: Vec<i32> = d.into_iter().copied().collect();
            apply_operations(&d2, &o)
        })
        .flatten()
        .sorted()
        .unique()
        .collect();

    consecutive_ceiling_from_results(&possible_numbers)
}

fn apply_operations(
    digits: &[i32],
    operations: &[&fn(Rational32, Rational32) -> Option<Rational32>],
) -> Vec<i32> {
    let a = Rational32::from(digits[0]);
    let b = Rational32::from(digits[1]);
    let c = Rational32::from(digits[2]);
    let d = Rational32::from(digits[3]);

    let mut result: Vec<i32> = Vec::new();

    let r1 = operations[0](a, b);
    let r2 = operations[1](c, d);
    if r1.is_some() && r2.is_some() {
        let r3 = operations[2](r1.unwrap(), r2.unwrap());
        if r3.is_some() {
            let r3 = r3.unwrap();
            if r3.is_integer() {
                result.push(r3.to_integer());
            }
        }
    }

    let r1 = operations[0](a, b);
    if r1.is_some() {
        let r2 = operations[1](r1.unwrap(), c);
        if r2.is_some() {
            let r3 = operations[2](r2.unwrap(), d);
            if r3.is_some() {
                let r3 = r3.unwrap();
                if r3.is_integer() {
                    result.push(r3.to_integer());
                }
            }
        }
    }

    result
}

fn consecutive_ceiling_from_results(results: &[i32]) -> i32 {
    for i in 1.. {
        if !results.contains(&i) {
            return i - 1;
        }
    }
    -1
}

fn solution() -> i64 {
    let mut max_ceiling = 0;
    let mut result: Vec<i32> = Vec::new();
    for subset in (0..10).combinations(4) {
        let ceiling = consecutive_ceiling_from_digits(&subset);
        if ceiling > max_ceiling {
            result = subset;
            max_ceiling = ceiling;
        }
    }
    let result: Vec<i64> = result.into_iter().map(Into::into).collect();
    int_from_digits(&result)
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 93,
        implementations: &[("", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consecutive_ceiling() {
        assert_eq!(consecutive_ceiling_from_results(&[-1, 1, 2, 3, 4, 6]), 4);
    }

    #[test]
    fn test_consecutive_ceiling_from_seeds() {
        assert_eq!(consecutive_ceiling_from_digits(&[1, 2, 3, 4]), 28);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 1_258);
    }
}
