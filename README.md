# Graph

A simple implementation of graph structures based on adjacency lists. It contains structures for both undirected and directed graphs.

## Functions

### For Both `UndirectedGraph` and `DirectedGraph`

- `new(order: usize) -> Self`: Initializes a new graph with `order` vertices.
- `add_edge(v: usize, w: usize)`: Adds an edge between vertices `v` and `w`.
- `delete_edge(v: usize, w: usize)`: Deletes the edge between vertices `v` and `w`.
- `has_edge(v: usize, w: usize) -> bool`: Returns `true` if there is an edge between vertices `v` and `w`, `false` otherwise.
- `order() -> usize`: Returns the number of vertices in the graph.
- `neighbors_of(v: usize) -> &Vec<usize>`: Returns a reference to a vector of vertices adjacent to vertex `v`.
- `size() -> usize`: Returns the number of edges in the graph.
- `clear()`: Removes all edges from the graph.

### For `UndirectedGraph` Only

- `degree(v: usize) -> usize`: Returns the degree (the number of edges connected to) of vertex `v`.

### For `DirectedGraph` Only

- `outdegree(v: usize) -> usize`: Returns the out-degree (the number of edges going out from) of vertex `v`.
- `indegree(v: usize) -> usize`: Returns the in-degree (the number of edges coming into) of vertex `v`.

## Usage

This is a basic example of how to use the `UndirectedGraph` and `DirectedGraph`:

```rust
use graph_rs::*;

// Create an undirected graph
let mut undirected_graph = UndirectedGraph::new(5);
undirected_graph.add_edge(0, 1);
undirected_graph.add_edge(0, 2);
println!("{}", undirected_graph.has_edge(1, 0)); // Outputs: true
println!("{}", undirected_graph.degree(0)); // Outputs: 2

// Create a directed graph
let mut directed_graph = DirectedGraph::new(5);
directed_graph.add_edge(0, 1);
directed_graph.add_edge(0, 2);
println!("{}", directed_graph.has_edge(1, 0)); // Outputs: false
println!("{}", directed_graph.outdegree(0)); // Outputs: 2
println!("{}", directed_graph.indegree(1)); // Outputs: 1
```
