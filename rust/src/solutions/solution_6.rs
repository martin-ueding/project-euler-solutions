fn solution() -> i64 {
    let mut sum_numbers: i64 = 0;
    let mut sum_squares = 0;
    for i in 1..101 {
        sum_numbers += i;
        sum_squares += i * i;
    }
    sum_numbers.pow(2) - sum_squares
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 6,
        solve: solution,
    }
}
