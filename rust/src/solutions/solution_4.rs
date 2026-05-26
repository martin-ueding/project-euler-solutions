use crate::digits::is_palindrome;

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
        implementations: &[("", solution)],
        solution: None,
    }
}
