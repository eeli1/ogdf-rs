use crate::graph_raw::GraphRaw;

extern "C" {
    fn randomBiconnectedGraph(n: i32, m: i32) -> GraphRaw;
    fn randomUpwardPlanarBiconnectedDigraph(n: i32, m: i32) -> GraphRaw;
    fn randomWattsStrogatzGraph(n: i32, k: i32, probability: f64) -> GraphRaw;
    fn randomWaxmanGraph(nodes: i32, alpha: f64, beta: f64, width: f64, height: f64) -> GraphRaw;
    fn randomTriconnectedGraph(n: i32, p1: f64, p2: f64) -> GraphRaw;
    fn randomTree(n: i32, max_deg: i32, max_width: i32) -> GraphRaw;
    fn randomSimpleGraphByProbability(n: i32, p_edge: f64) -> GraphRaw;
    fn randomSimpleGraph(n: i32, m: i32) -> GraphRaw;
    fn randomSimpleConnectedGraph(n: i32, m: i32) -> GraphRaw;
    fn randomSeriesParallelDAG(edges: i32, p: f64, flt: f64) -> GraphRaw;
    fn randomRegularGraph(n: i32, d: i32) -> GraphRaw;
    fn randomPlanarBiconnectedDigraph(n: i32, m: i32, p: f64, multi_edges: bool) -> GraphRaw;
    fn randomHierarchy(
        n: i32,
        m: i32,
        planar: bool,
        single_source: bool,
        long_edges: bool,
    ) -> GraphRaw;
    fn randomGraph(n: i32, m: i32) -> GraphRaw;
    fn randomGeometricCubeGraph(nodes: i32, threshold: f64, dimension: i32) -> GraphRaw;
    fn randomDigraph(n: i32, p: f64) -> GraphRaw;
    fn setSeed(val: i32);
}

/// **Parameters:**
///
/// `n` – is the number of nodes of the generated graph.
///
/// `m` – is the number of edges of the generated graph.
pub fn biconnected(n: i32, m: i32) -> GraphRaw {
    unsafe { randomBiconnectedGraph(n, m) }
}

/// **Parameters:**
///
/// `n` – is the number of nodes of the generated graph.
///
/// `m` – is the number of edges of the generated graph.
pub fn upward_planar_biconnected_digraph(n: i32, m: i32) -> GraphRaw {
    unsafe { randomUpwardPlanarBiconnectedDigraph(n, m) }
}

/// Takes a regular lattice graph and, with given probability, rewires each edge to a random other non-neighbor. Collective dynamics of ‘small-world’ networks https://www.nature.com/articles/30918.pdf
///
///
/// **Parameters:**
///
/// `n` – is the number of nodes of the generated graph.
///
/// `k` – is the initial degree of each node and must be even and smaller than half of n.
///
/// `probability` – determines how likely each edge is rewired. A probability of 0 will not modify the graph, while one of 1 will cause full randomness.
pub fn watts_strogatz(n: i32, k: i32, probability: f64) -> GraphRaw {
    unsafe { randomWattsStrogatzGraph(n, k, probability) }
}

/// Routing of Multipoint Connections Bernard M. Waxman (1988) After generating the nodes, edges are inserted between each pair of nodes v, w with probability based on their euclidean distance
///
///
/// **Parameters:**
///
/// `nodes` – is the number of nodes of the generated graph.
///
/// `alpha` – is a parameter for the probability in the range (0,1]. Small values increase the density of short edges relative to longer ones.
///
/// `beta` – is a parameter for the probability in the range (0,1]. Large values result in a graph with higher edge density.
///
/// `width` – is the width of the area the nodes are distributed in.
///
/// `height` – is the height of the area the nodes are distributed in.
pub fn waxman(nodes: i32, alpha: f64, beta: f64, width: f64, height: f64) -> GraphRaw {
    unsafe { randomWaxmanGraph(nodes, alpha, beta, width, height) }
}

/// The graph generator proceeds as follows. It starts with a K_4 and performs then n -4 split node operations on randomly selected nodes of the graph constructed so far. Each such operation splits a node v into two nodes x and y and distributes v's neighbors to the two nodes such that each node gets at least two neighbors. Additionally, the edge (x,y) is inserted. The neighbors are distributed such that a neighbor of v becomes - only a neighbor of x with probability p1; - only a neighbor of y with probability p1; - a neighbor of both x and y with probability 1.0 - p1 - p2.
///
/// **Parameters:**
///
/// `n` – is the number of nodes in the generated graph.
///
/// `p1` – is the probability that an edge is moved only to the left node after splitting a node.
///
/// `p2` – is the probability that an edge is moved only to the right node after splitting a node. The probability for a neighbor to be moved to both split nodes is 1.0 - p1 - p2. The higher this probability, the higher the density of the resulting graph.
pub fn triconnected(n: i32, p1: f64, p2: f64) -> GraphRaw {
    unsafe { randomTriconnectedGraph(n, p1, p2) }
}

/// **Parameters:**
///
/// `n` – is the number of nodes of the tree.
///
/// `max_deg` – is the maximal allowed node degree; 0 means no restriction.
///
/// `max_width` – is the maximal allowed width of a level; 0 means no restriction.
pub fn tree(n: i32, max_deg: i32, max_width: i32) -> GraphRaw {
    unsafe { randomTree(n, max_deg, max_width) }
}

/// Algorithm based on PreZER/LogZER from: Sadegh Nobari, Xuesong Lu, Panagiotis Karras, and Stéphane Bressan. 2011. Fast random graph generation. In Proceedings of the 14th International Conference on Extending Database Technology (EDBT/ICDT '11), ACM, New York, NY, USA, 331-342. DOI=http://dx.doi.org/10.1145/1951365.1951406
///
///
/// **Parameters:**
///
/// `n` – is the number of nodes of the generated graph.
///
/// `p_edge` – is the probability for each edge to be added into the graph.
pub fn simple_graph_by_probability(n: i32, p_edge: f64) -> GraphRaw {
    unsafe { randomSimpleGraphByProbability(n, p_edge) }
}

/// **Parameters:**
///
/// n – is the number of nodes of the generated graph.
///
/// m – is the number of edges of the generated graph.
pub fn simple(n: i32, m: i32) -> GraphRaw {
    unsafe { randomSimpleGraph(n, m) }
}

/// **Parameters:**
///
/// n – is the number of nodes of the generated graph.
///
/// m – is the number of edges of the generated graph.
pub fn simple_connected(n: i32, m: i32) -> GraphRaw {
    unsafe { randomSimpleConnectedGraph(n, m) }
}

/// This function creates a random series parallel biconnected DAG. Note, that the resulting graph is trivially upward planar! To use this generator for experiments, e.g. concerning upward planarity, you can fit the graph by reversing some edges with the parameter 0 < flt < 1.
///
///
/// **Parameters:**
///
/// `edges` – is the number of edges in the generated graph.
///
/// `p` – = probability of a series composition; default = 0.5
///
/// `flt` – = up to edges*flt edges will be reversed preversing acyclicity; default = 0.0
pub fn series_parallel_dag(edges: i32, p: f64, flt: f64) -> GraphRaw {
    unsafe { randomSeriesParallelDAG(edges, p, flt) }
}

/// **Parameters:**
///
/// `n` – is the number of nodes of the generated graph.
///
/// `d` – is the degree of each vertex
pub fn regular(n: i32, d: i32) -> GraphRaw {
    unsafe { randomRegularGraph(n, d) }
}

/// **Parameters:**
///
/// `n` – is the number of nodes of the generated graph.
///
/// `m` – is the number of edges of the generated graph.
///
/// `p` – up to m * p edges will be reversed preversing acyclicity; default = 0.0.
///
/// `multi_edges` – determines if the generated graph may contain multi-edges; default = false.
pub fn planar_biconnected_digraph(n: i32, m: i32, p: f64, multi_edges: bool) -> GraphRaw {
    unsafe { randomPlanarBiconnectedDigraph(n, m, p, multi_edges) }
}

/// **Parameters:**
///
/// `n` – is the number of nodes.
///
/// `m` – is the number of edges.
///
/// `planar` – determines if the resulting graph is (level-)planar.
///
/// `single_source` – determines if the graph is a single-source graph.
///
/// `long_edges` – determines if the graph has long edges (spanning 2 layers or more); otherwise the graph is proper.
pub fn hierarchy(n: i32, m: i32, planar: bool, single_source: bool, long_edges: bool) -> GraphRaw {
    unsafe { randomHierarchy(n, m, planar, single_source, long_edges) }
}

/// **Parameters:**
///
/// `n` – is the number of nodes of the generated graph.
///
/// `m` – is the number of edges of the generated graph.
pub fn graph(n: i32, m: i32) -> GraphRaw {
    unsafe { randomGraph(n, m) }
}

/// **Parameters:**
///
/// `nodes` – is the number of nodes of the generated graph.
///
/// `threshold` – is threshold radius of nodes which will be connected.
///
/// `dimension` – is the dimension of the cube.
pub fn geometric_cube(nodes: i32, threshold: f64, dimension: i32) -> GraphRaw {
    unsafe { randomGeometricCubeGraph(nodes, threshold, dimension) }
}

/// **Parameters:**
///
/// `n` – is the number of nodes in the generated graph.
///
/// `p` – is the probability that an edge is created (for each node pair)
pub fn digraph(n: i32, p: f64) -> GraphRaw {
    unsafe { randomDigraph(n, p) }
}

/// Sets the seed for functions like randomSeed(), randomNumber(), randomDouble().
pub fn set_seed(val: i32) {
    unsafe { setSeed(val) }
}
