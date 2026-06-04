/// Verifies a solution for $x^2 - D y^2 = c$.
pub fn solves_diophantine_equation(x: i64, y: i64, d: i64, c: i64) -> bool {
    x * x - d * y * y == c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solves_diophantine_equation() {
        assert!(solves_diophantine_equation(2, 0, 12, 4));
    }
}
