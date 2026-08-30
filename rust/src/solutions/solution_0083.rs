use crate::{
    graphs::min_path_sum,
    matrix_graphs::{AllowedDirection::AllDirections, MatrixGraph},
};

fn solution() -> i64 {
    let graph = MatrixGraph::new("../data/0083_matrix.txt", AllDirections);
    min_path_sum(&graph, 0, graph.lower_right_vertex_id()) as i64
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 83,
        implementations: &[("", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_is_correct() {
        assert_eq!(solution(), 425185);
    }
}
