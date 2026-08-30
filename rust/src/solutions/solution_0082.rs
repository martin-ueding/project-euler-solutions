use crate::{
    graphs::min_path_sum,
    matrix_graphs::{AllowedDirection::RightDownUp, LEFT_EDGE_ID, MatrixGraph, RIGHT_EDGE_ID},
};

fn solution() -> i64 {
    let graph = MatrixGraph::new("../data/0083_matrix.txt", RightDownUp);
    min_path_sum(&graph, LEFT_EDGE_ID, RIGHT_EDGE_ID) as i64
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 82,
        implementations: &[("", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_is_correct() {
        assert_eq!(solution(), 260324);
    }
}
