use crate::primes::greatest_common_denominator;

/// Expand square root as continuous fraction.
///
/// The function returns the coefficients in the expansion. The first vector in the tuple is the leading part, the second part is the periodic part.
pub fn expand_root(number: i64) -> (Vec<i64>, Vec<i64>) {
    let floor = number.isqrt();
    if floor.pow(2) == number {
        return (vec![floor], vec![]);
    }
    let mut results: Vec<i64> = vec![floor];
    let mut states: Vec<(i64, i64)> = vec![(1, floor)];
    let mut c = floor;
    let mut b = 1;
    let mut state;
    loop {
        let d = number - c.pow(2);
        let gcd = greatest_common_denominator(b, d);
        b /= gcd;
        let d = d / gcd;
        let split = (floor + c) / d;
        let a = split * b;
        c -= split * d;
        c = -c;
        b = d;
        state = (b, c);
        results.push(a);
        if states.contains(&state) {
            break;
        }
        states.push(state);
    }
    let i = states.iter().position(|&s| s == state).unwrap_or(0) + 1;
    (results[..i].to_vec(), results[i..].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_root() {
        assert_eq!(expand_root(2), (vec![1], vec![2]));
        assert_eq!(expand_root(3), (vec![1], vec![1, 2]));
        assert_eq!(expand_root(5), (vec![2], vec![4]));
        assert_eq!(expand_root(6), (vec![2], vec![2, 4]));
        assert_eq!(expand_root(7), (vec![2], vec![1, 1, 1, 4]));
        assert_eq!(expand_root(8), (vec![2], vec![1, 4]));
        assert_eq!(expand_root(10), (vec![3], vec![6]));
        assert_eq!(expand_root(11), (vec![3], vec![3, 6]));
        assert_eq!(expand_root(12), (vec![3], vec![2, 6]));
        assert_eq!(expand_root(13), (vec![3], vec![1, 1, 1, 1, 6]));
        assert_eq!(expand_root(23), (vec![4], vec![1, 3, 1, 8]));
    }
}
