extern crate terrain_graph;
use terrain_graph::edge_attributed_directed::EdgeAttributedDirectedGraph;

#[test]
fn test_new() {
    let g: EdgeAttributedDirectedGraph<i32> = EdgeAttributedDirectedGraph::new(3);
    assert_eq!(g.order(), 3);
    assert_eq!(g.size(), 0);
}

#[test]
fn test_add_edge() {
    let mut g: EdgeAttributedDirectedGraph<i32> = EdgeAttributedDirectedGraph::new(3);
    g.add_edge(0, 1, 10);
    g.add_edge(0, 2, 20);
    assert_eq!(g.size(), 2);
    assert_eq!(g.has_edge(0, 1), (true, 10));
    assert_eq!(g.has_edge(1, 0), (false, 0));
    assert_eq!(g.has_edge(0, 2), (true, 20));
    assert_eq!(g.has_edge(2, 0), (false, 0));
}

#[test]
fn test_delete_edge() {
    let mut g: EdgeAttributedDirectedGraph<i32> = EdgeAttributedDirectedGraph::new(3);
    g.add_edge(0, 1, 10);
    g.delete_edge(0, 1);
    assert_eq!(g.size(), 0);
    assert_eq!(g.has_edge(0, 1), (false, 0));
}

#[test]
fn test_clear() {
    let mut g: EdgeAttributedDirectedGraph<i32> = EdgeAttributedDirectedGraph::new(3);
    g.add_edge(0, 1, 10);
    g.add_edge(1, 2, 20);
    g.clear();
    assert_eq!(g.order(), 3);
    assert_eq!(g.size(), 0);
    assert_eq!(g.has_edge(0, 1), (false, 0));
    assert_eq!(g.has_edge(1, 2), (false, 0));
}

#[test]
fn test_neighbors_of() {
    let mut g: EdgeAttributedDirectedGraph<i32> = EdgeAttributedDirectedGraph::new(3);
    g.add_edge(0, 1, 10);
    g.add_edge(0, 2, 20);
    let neighbors_0 = g.neighbors_of(0);
    assert_eq!(neighbors_0.len(), 2);
    assert_eq!(neighbors_0[0].0, 1);
    assert_eq!(neighbors_0[0].1, 10);
    assert_eq!(neighbors_0[1].0, 2);
    assert_eq!(neighbors_0[1].1, 20);
    let neighbors_1 = g.neighbors_of(1);
    assert_eq!(neighbors_1.len(), 0);
    let neighbors_2 = g.neighbors_of(2);
    assert_eq!(neighbors_2.len(), 0);
}

#[test]
fn test_indegree_outdegree() {
    let mut g: EdgeAttributedDirectedGraph<i32> = EdgeAttributedDirectedGraph::new(3);
    g.add_edge(0, 1, 10);
    g.add_edge(0, 2, 20);
    assert_eq!(g.outdegree(0), 2);
    assert_eq!(g.indegree(0), 0);
    assert_eq!(g.outdegree(1), 0);
    assert_eq!(g.indegree(1), 1);
    assert_eq!(g.outdegree(2), 0);
    assert_eq!(g.indegree(2), 1);
}
