/// An edge-attributed directed graph.
pub struct EdgeAttributedDirectedGraph<Attr: Copy + Clone + Default> {
    adjacency_list: Vec<Vec<(usize, Attr)>>,
    edge_count: usize,
}

impl<Attr: Copy + Clone + Default> EdgeAttributedDirectedGraph<Attr> {
    /// Initialize an empty graph with order vertices.
    pub fn new(order: usize) -> Self {
        EdgeAttributedDirectedGraph {
            adjacency_list: vec![vec![]; order],
            edge_count: 0,
        }
    }

    /// Add an edge from vertex v to vertex w, with a specified attribute.
    pub fn add_edge(&mut self, v: usize, w: usize, attr: Attr) {
        self.adjacency_list[v].push((w, attr));
        self.edge_count += 1;
    }

    /// Remove the edge from vertex v to vertex w.
    pub fn delete_edge(&mut self, v: usize, w: usize) {
        self.adjacency_list[v].retain(|&(x, _)| x != w);
        self.edge_count -= 1;
    }

    /// Check if there is an edge from vertex v to vertex w.
    pub fn has_edge(&self, v: usize, w: usize) -> (bool, Attr) {
        for &(x, attr) in self.adjacency_list[v].iter() {
            if x == w {
                return (true, attr);
            }
        }
        (false, Default::default())
    }

    /// Return the number of vertices in the graph.
    pub fn order(&self) -> usize {
        self.adjacency_list.len()
    }

    /// Return a reference to the adjacency list of vertex v.
    pub fn neighbors_of(&self, v: usize) -> &Vec<(usize, Attr)> {
        self.adjacency_list[v].as_ref()
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

    /// Return the out-degree of vertex v.
    pub fn outdegree(&self, v: usize) -> usize {
        self.adjacency_list[v].len()
    }

    /// Return the in-degree of vertex v.
    ///
    /// Note: This requires O(|V| + |E|) time to compute.
    pub fn indegree(&self, v: usize) -> usize {
        self.adjacency_list
            .iter()
            .filter(|&adj| adj.iter().any(|&(x, _)| x == v))
            .count()
    }
}
