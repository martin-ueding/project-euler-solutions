use crate::graphs::Graph;
use crate::{
    graphs::min_distance,
    triangle_graphs::{TriangleGraph, VIRTUAL_END_VERTEX_ID, load_triangle_numbers},
};

fn solution() -> i64 {
    let numbers = load_triangle_numbers("../data/0018_triangle.txt");
    let graph = TriangleGraph { lines: numbers };
    -min_distance(
        &graph,
        &graph.vertex(&0),
        &graph.vertex(&VIRTUAL_END_VERTEX_ID),
    ) as i64
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 18,
        implementations: &[("", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_is_correct() {
        assert_eq!(solution(), 1074);
    }
}
