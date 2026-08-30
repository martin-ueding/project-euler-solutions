use crate::triangle_graphs::triangle_max_path;

fn solution() -> i64 {
    triangle_max_path("../data/0067_triangle.txt")
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 67,
        implementations: &[("", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_is_correct() {
        assert_eq!(solution(), 7273);
    }
}
