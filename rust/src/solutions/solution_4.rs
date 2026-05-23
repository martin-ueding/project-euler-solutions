fn is_palindrome(number: i64) -> bool {
    let reversed: String = number.to_string().chars().rev().collect();
    number.to_string() == reversed
}

fn solution() -> i64 {
    let mut result: i64 = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if product > result && is_palindrome(product) {
                result = product;
            }
        }
    }
    result
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 4,
        solve: solution,
    }
}
