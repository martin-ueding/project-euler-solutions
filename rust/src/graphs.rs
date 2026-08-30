use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub type VertexID = usize;
pub type Weight = i32;

#[derive(Clone, Copy)]
pub struct Edge {
    from: VertexID,
    to: VertexID,
    weight: Weight,
}

impl Edge {
    pub fn new(from: VertexID, to: VertexID, weight: Weight) -> Self {
        Edge { from, to, weight }
    }
}

#[derive(Clone, Copy)]
pub struct Vertex {
    id: VertexID,
    weight: Weight,
}

impl Vertex {
    pub fn new(id: VertexID, weight: Weight) -> Self {
        Vertex { id, weight }
    }
}

pub trait Graph {
    fn edges(&self, id: VertexID) -> Vec<Edge>;
    fn vertex(&self, id: VertexID) -> Vertex;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct UnvisitedVertex {
    total_distance: Weight,
    id: VertexID,
}

pub fn min_path_sum(g: &dyn Graph, start: VertexID, target: VertexID) -> Weight {
    let start = g.vertex(start);
    let mut unvisited_vertices: BinaryHeap<Reverse<UnvisitedVertex>> = BinaryHeap::new();
    unvisited_vertices.push(Reverse(UnvisitedVertex {
        total_distance: start.weight,
        id: start.id,
    }));
    let mut total_distances: HashMap<VertexID, Weight> = HashMap::new();
    total_distances.insert(start.id, start.weight);
    while !unvisited_vertices.is_empty() {
        let cur = unvisited_vertices
            .pop()
            .expect("non-empty due to above check")
            .0;
        for edge in g.edges(cur.id) {
            let vertex = g.vertex(edge.to);
            let distance = cur.total_distance + edge.weight + vertex.weight;
            // Unless a smaller distance to that vertex is already known, insert the computed distance and update the unvisited heap.
            if !total_distances
                .get(&vertex.id)
                .is_some_and(|d| *d < distance)
            {
                total_distances.insert(vertex.id, distance);
                unvisited_vertices.push(Reverse(UnvisitedVertex {
                    total_distance: distance,
                    id: vertex.id,
                }))
            }
        }
    }
    *total_distances
        .get(&target)
        .expect("target should be reachable from start")
}

pub struct ExplicitGraph {
    vertices: HashMap<VertexID, Vertex>,
    edges: HashMap<VertexID, Vec<Edge>>,
}

impl ExplicitGraph {
    pub fn new(vertices: &[Vertex], edges: &[Edge]) -> Self {
        let mut vertex_map: HashMap<VertexID, Vertex> = HashMap::new();
        let mut edge_map: HashMap<VertexID, Vec<Edge>> = HashMap::new();
        for vertex in vertices {
            vertex_map.insert(vertex.id, vertex.clone());
        }
        for edge in edges {
            edge_map.entry(edge.from).or_default().push(*edge);
        }
        ExplicitGraph {
            vertices: vertex_map,
            edges: edge_map,
        }
    }
}

impl Graph for ExplicitGraph {
    fn edges(&self, v: VertexID) -> Vec<Edge> {
        self.edges.get(&v).cloned().unwrap_or_default()
    }

    fn vertex(&self, id: VertexID) -> Vertex {
        *self
            .vertices
            .get(&id)
            .expect("Given vertex id must be contained in graph.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra_solves_explicit_graph() {
        let graph = ExplicitGraph::new(
            &vec![
                Vertex::new(1, 2),
                Vertex::new(2, 2),
                Vertex::new(3, 2),
                Vertex::new(4, 2),
            ],
            &vec![
                Edge::new(1, 2, 1),
                Edge::new(1, 3, 10),
                Edge::new(2, 3, 2),
                Edge::new(2, 4, 10),
                Edge::new(3, 4, 3),
            ],
        );

        assert_eq!(min_path_sum(&graph, 1, 4), 14)
    }
}
