extern crate terrain_graph;
use terrain_graph::edge_attributed_undirected::EdgeAttributedUndirectedGraph;

#[test]
fn test_new() {
    let g: EdgeAttributedUndirectedGraph<i32> = EdgeAttributedUndirectedGraph::new(3);
    assert_eq!(g.order(), 3);
    assert_eq!(g.size(), 0);
}

#[test]
fn test_add_edge() {
    let mut g: EdgeAttributedUndirectedGraph<i32> = EdgeAttributedUndirectedGraph::new(3);
    g.add_edge(0, 1, 10);
    g.add_edge(0, 2, 20);
    assert_eq!(g.size(), 2);
    assert_eq!(g.has_edge(0, 1), (true, Some(10)));
    assert_eq!(g.has_edge(1, 0), (true, Some(10)));
    assert_eq!(g.has_edge(0, 2), (true, Some(20)));
    assert_eq!(g.has_edge(2, 0), (true, Some(20)));
}

#[test]
fn test_delete_edge() {
    let mut g: EdgeAttributedUndirectedGraph<i32> = EdgeAttributedUndirectedGraph::new(3);
    g.add_edge(0, 1, 10);
    g.delete_edge(0, 1);
    assert_eq!(g.size(), 0);
    assert_eq!(g.has_edge(0, 1), (false, None));
}

#[test]
fn test_clear() {
    let mut g: EdgeAttributedUndirectedGraph<i32> = EdgeAttributedUndirectedGraph::new(3);
    g.add_edge(0, 1, 10);
    g.add_edge(1, 2, 20);
    g.clear();
    assert_eq!(g.order(), 3);
    assert_eq!(g.size(), 0);
    assert_eq!(g.has_edge(0, 1), (false, None));
    assert_eq!(g.has_edge(1, 2), (false, None));
}

#[test]
fn test_degree() {
    let mut g: EdgeAttributedUndirectedGraph<i32> = EdgeAttributedUndirectedGraph::new(3);
    g.add_edge(0, 1, 10);
    g.add_edge(0, 2, 20);
    assert_eq!(g.degree(0), 2);
    assert_eq!(g.degree(1), 1);
    assert_eq!(g.degree(2), 1);
}