//! Cube Digit Pairs (Problem 90).

use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::Itertools;

const SQUARES: &'static [(u16, u16)] = &[
    (0, 1),
    (0, 4),
    (0, 9),
    (1, 6),
    (2, 5),
    (3, 6),
    (4, 9),
    (6, 4),
    (8, 1),
];

fn set_to_binary(numbers: &[u16]) -> u16 {
    let mut result = 0;
    for number in numbers {
        result += 1 << number;
    }
    result
}

fn enable_69(mask: u16) -> u16 {
    if mask & 0b0000_0010_0100_0000 > 0 {
        mask | 0b0000_0010_0100_0000
    } else {
        mask
    }
}

fn can_represent(mask: u16, digit: u16) -> bool {
    mask & (1 << digit) > 0
}

fn represents_number(number: (u16, u16), mask1: u16, mask2: u16) -> bool {
    can_represent(mask1, number.0) && can_represent(mask2, number.1)
        || can_represent(mask1, number.1) && can_represent(mask2, number.0)
}

fn represents_all_numbers(mask1: u16, mask2: u16) -> bool {
    let mask1 = enable_69(mask1);
    let mask2 = enable_69(mask2);
    SQUARES
        .iter()
        .all(|&square| represents_number(square, mask1, mask2))
}

fn solution() -> i64 {
    let pool: Vec<u16> = (0..10).collect();
    let mut valid_cubes: HashSet<(u16, u16)> = HashSet::new();
    for combination1 in pool.iter().copied().combinations(6) {
        for combination2 in pool.iter().copied().combinations(6) {
            let mask1 = set_to_binary(&combination1);
            let mask2 = set_to_binary(&combination2);
            if represents_all_numbers(mask1, mask2) {
                valid_cubes.insert((min(mask1, mask2), max(mask1, mask2)));
            }
        }
    }
    valid_cubes.len() as i64
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 90,
        implementations: &[("testing all combinations", solution)],
        solution: Some(1217),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_to_binary() {
        let expected: u16 = 0b0000_0011_1110_0001;
        let actual = set_to_binary(&[0, 5, 6, 7, 8, 9]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_enable_69_present() {
        let expected: u16 = 0b0000_0011_1110_0001;
        let actual = enable_69(0b0000_0011_1010_0001);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_enable_69_missing() {
        let expected: u16 = 0b0000_0001_1010_0001;
        let actual = enable_69(0b0000_0001_1010_0001);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_can_represent() {
        let mask: u16 = 0b0000_0001_1010_0001;
        assert!(can_represent(mask, 5));
        assert!(!can_represent(mask, 6));
    }

    #[test]
    fn test_represents_number() {
        let set1 = &[0, 5, 6, 7, 8, 9];
        let set2 = &[1, 2, 3, 4, 8, 9];

        let mask1 = enable_69(set_to_binary(set1));
        let mask2 = enable_69(set_to_binary(set2));

        assert!(represents_all_numbers(mask1, mask2));
    }

    
    #[test]
    fn test_solution() {
        assert_eq!(solution(), 1_217);
    }
}
