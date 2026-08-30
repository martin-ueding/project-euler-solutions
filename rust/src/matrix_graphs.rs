use std::fs;

use crate::{
    graphs::{Edge, Graph, Vertex, VertexID},
    matrix_graphs::AllowedDirection::{AllDirections, RightDown},
};

#[derive(PartialEq)]
pub enum AllowedDirection {
    RightDown,
    RightDownUp,
    AllDirections,
}

pub struct MatrixGraph {
    numbers: Vec<Vec<i32>>,
    allowed_directions: AllowedDirection,
}

impl MatrixGraph {
    pub fn new(path: &str, allowed_directions: AllowedDirection) -> Self {
        MatrixGraph {
            numbers: read_matrix_file(path),
            allowed_directions,
        }
    }

    pub fn lower_right_vertex_id(&self) -> VertexID {
        self.num_rows() * self.num_cols() - 1
    }

    fn num_rows(&self) -> usize {
        self.numbers.len()
    }

    fn num_cols(&self) -> usize {
        self.numbers[0].len()
    }
}

fn read_matrix_file(path: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(path)
        .expect("file under `path` should exist")
        .lines()
        .map(line_to_integers)
        .collect()
}

fn line_to_integers(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect()
}

pub const LEFT_EDGE_ID: VertexID = 1_000_001;
pub const RIGHT_EDGE_ID: VertexID = 1_000_002;

impl Graph for MatrixGraph {
    fn edges(&self, id: VertexID) -> Vec<Edge> {
        let mut edges: Vec<Edge> = Vec::new();
        if id == LEFT_EDGE_ID {
            for row in 0..self.numbers.len() {
                edges.push(Edge::new(id, row * self.numbers.len(), 0));
            }
        } else if id == RIGHT_EDGE_ID {
        } else {
            if id / self.numbers.len() < self.numbers.len() - 1 {
                edges.push(Edge::new(id, id + self.numbers.len(), 0));
            }
            if id % self.numbers.len() < self.numbers.len() - 1 {
                edges.push(Edge::new(id, id + 1, 0));
            } else {
                edges.push(Edge::new(id, RIGHT_EDGE_ID, 0));
            }
            if self.allowed_directions != RightDown && id / self.numbers.len() > 0 {
                edges.push(Edge::new(id, id - self.numbers.len(), 0));
            }
            if self.allowed_directions == AllDirections && id % self.numbers.len() > 0 {
                edges.push(Edge::new(id, id - 1, 0));
            }
        }
        edges
    }

    fn vertex(&self, id: VertexID) -> Vertex {
        match id {
            LEFT_EDGE_ID => Vertex::new(LEFT_EDGE_ID, 0),
            RIGHT_EDGE_ID => Vertex::new(RIGHT_EDGE_ID, 0),
            _ => Vertex::new(
                id,
                self.numbers[id / self.numbers.len()][id % self.numbers.len()],
            ),
        }
    }
}
