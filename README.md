# terrain-graph

A simple graph library for Rust based on adjacency lists.

This is a subproject for the [fastlem](https://github.com/TadaTeruki/fastlem).

This library contains the following structures:
 - `DirectedGraph`
 - `UndirectedGraph`
 - `EdgeAttributedDirectedGraph`
 - `EdgeAttributedUndirectedGraph`

## Usage

```Cargo.toml
[dependencies]
terrain-graph = "1.0.1"
```

This is a basic example of how to use the `UndirectedGraph` and `DirectedGraph`:

```rust
use terrain_graph::*;

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
