fn solution_division_check() -> i64 {
    let mut sum_of_multiples = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum_of_multiples += i;
        }
    }
    sum_of_multiples
}

fn solution_division_check_iterator() -> i64 {
    (1..1000).filter(|&i| i % 3 == 0 || i % 5 == 0).sum()
}

fn sum_of_natural_numbers(end: i64, step: i64) -> i64 {
    let count = end / step + 1;
    count * (count - 1) * step / 2
}

fn solution_closed_form() -> i64 {
    sum_of_natural_numbers(999, 3) + sum_of_natural_numbers(999, 5)
        - sum_of_natural_numbers(999, 15)
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 1,
        implementations: &[("division check loop", solution_division_check), ("division check iterator", solution_division_check_iterator), ("closed form", solution_closed_form)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_division_check() {
        assert_eq!(solution_division_check(), 233_168);
    }

    #[test]
    fn test_solution_division_check_iterator() {
        assert_eq!(solution_division_check_iterator(), 233_168);
    }

    #[test]
    fn test_solution_closed_form() {
        assert_eq!(solution_closed_form(), 233_168);
    }
}
