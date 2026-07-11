use crate::fibonacci::FibonacciIterator;

fn solution() -> i64 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 1;

    loop {
        if a > 4_000_000 {
            break;
        }
        if a % 2 == 0 {
            sum += a;
        }
        let c = a + b;
        a = b;
        b = c;
    }

    sum
}

fn solution_iterator() -> i64 {
    FibonacciIterator::new()
        .take_while(|&f| f <= 4_000_000)
        .filter(|&f| f % 2 == 0)
        .sum()
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 2,
        implementations: &[("loop", solution), ("iterator", solution_iterator)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 4_613_732);
    }

    #[test]
    fn solution_iterator_is_correct() {
        assert_eq!(solution_iterator(), 4_613_732);
    }
}
