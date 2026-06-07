fn solution_division_check() -> i64 {
    let mut sum_of_multiples = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum_of_multiples += i;
        }
    }
    sum_of_multiples
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 1,
        implementations: &[("division check", solution_division_check)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_division_check() {
        assert_eq!(solution_division_check(), 233_168);
    }
}
