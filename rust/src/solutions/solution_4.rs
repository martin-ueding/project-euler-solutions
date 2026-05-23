fn is_palindrome(number: i64) -> bool {
    let mut reversed: i64 = 0;
    let mut remainder = number;
    while remainder > 0 {
        reversed *= 10;
        let last_digit = remainder % 10;
        reversed += last_digit;
        remainder /= 10;
    }
    number == reversed
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
