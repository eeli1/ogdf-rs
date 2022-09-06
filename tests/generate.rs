#[test]
fn regular_tree() {
    let gr = ogdf_rs::generate::regular_tree(5, 2);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn regular_lattice() {
    let gr = ogdf_rs::generate::regular_lattice(8, 4);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn petersen() {
    let gr = ogdf_rs::generate::petersen(5, 2);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn grid() {
    let gr = ogdf_rs::generate::grid(5, 2, false, false);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn empty() {
    let gr = ogdf_rs::generate::empty(5);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn cube() {
    let gr = ogdf_rs::generate::cube(5);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn complete_k_partite() {
    let gr = ogdf_rs::generate::complete_k_partite();
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn complete() {
    let gr = ogdf_rs::generate::complete(5);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn complete_bipartite() {
    let gr = ogdf_rs::generate::complete_bipartite(5, 2);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn circulant() {
    let gr = ogdf_rs::generate::circulant(5);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn biconnected() {
    let gr = ogdf_rs::generate::random::biconnected(5, 3);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn upward_planar_biconnected_digraph() {
    let gr = ogdf_rs::generate::random::upward_planar_biconnected_digraph(5, 3);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn watts_strogatz() {
    let gr = ogdf_rs::generate::random::watts_strogatz(8, 4, 0.5);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn waxman() {
    let gr = ogdf_rs::generate::random::waxman(15, 0.5, 0.5, 1., 1.);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn triconnected() {
    let gr = ogdf_rs::generate::random::triconnected(15, 0.3, 0.2);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn tree() {
    let gr = ogdf_rs::generate::random::tree(25, 4, 5);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn simple_graph_by_probability() {
    let gr = ogdf_rs::generate::random::simple_graph_by_probability(5, 0.4);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn simple() {
    let gr = ogdf_rs::generate::random::simple(5, 3);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn simple_connected() {
    let gr = ogdf_rs::generate::random::simple_connected(5, 8);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn series_parallel_dag() {
    let gr = ogdf_rs::generate::random::series_parallel_dag(5, 0.5, 0.0);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn regular() {
    let gr = ogdf_rs::generate::random::regular(5, 2);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn planar_biconnected_digraph() {
    let gr = ogdf_rs::generate::random::planar_biconnected_digraph(5, 3, 0.0, false);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn hierarchy() {
    let gr = ogdf_rs::generate::random::hierarchy(5, 3, false, false, false);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn graph() {
    let gr = ogdf_rs::generate::random::graph(5, 3);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn geometric_cube() {
    let gr = ogdf_rs::generate::random::geometric_cube(5, 3.0, 2);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}

#[test]
fn digraph() {
    let gr = ogdf_rs::generate::random::digraph(5, 0.4);
    assert_ne!(gr.graph_type, ogdf_rs::GraphType::Error);
}
