/// A directed graph.
#[derive(Clone)]
pub struct DirectedGraph {
    adjacency_list: Vec<Vec<usize>>,
    edge_count: usize,
}

impl DirectedGraph {
    /// Initialize an empty graph with order vertices.
    pub fn new(order: usize) -> Self {
        DirectedGraph {
            adjacency_list: vec![vec![]; order],
            edge_count: 0,
        }
    }

    /// Add an edge from v to w.
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adjacency_list[v].push(w);
        self.edge_count += 1;
    }

    /// Remove the edge from v to w.
    pub fn delete_edge(&mut self, v: usize, w: usize) {
        self.adjacency_list[v].retain(|&x| x != w);
        self.edge_count -= 1;
    }

    /// Check if there is an edge from v to w.
    pub fn has_edge(&self, v: usize, w: usize) -> bool {
        self.adjacency_list[v].contains(&w)
    }

    /// Return the number of vertices in the graph.
    pub fn order(&self) -> usize {
        self.adjacency_list.len()
    }

    /// Return a reference to the adjacency list of vertex v.
    pub fn neighbors_of(&self, v: usize) -> &Vec<usize> {
        &self.adjacency_list[v]
    }

    /// Return the number of edges in the graph.
    pub fn size(&self) -> usize {
        self.edge_count
    }

    /// Remove all edges from the graph.
    pub fn clear(&mut self) {
        for v in self.adjacency_list.iter_mut() {
            v.clear();
        }
        self.edge_count = 0;
    }

    /// Return the outdegree of vertex v.
    pub fn outdegree(&self, v: usize) -> usize {
        self.adjacency_list[v].len()
    }

    /// Return the indegree of vertex v.
    ///
    /// Note: This requires O(|V| + |E|) time to compute.
    pub fn indegree(&self, v: usize) -> usize {
        self.adjacency_list
            .iter()
            .filter(|&adj| adj.contains(&v))
            .count()
    }
}
