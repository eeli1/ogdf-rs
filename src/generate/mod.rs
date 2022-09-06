pub mod random;
use crate::graph_raw::GraphRaw;

extern "C" {
    fn regularTree(n: i32, children: i32) -> GraphRaw;
    fn regularLatticeGraph(n: i32, k: i32) -> GraphRaw;
    fn petersenGraph(n: i32, m: i32) -> GraphRaw;
    fn gridGraph(n: i32, m: i32, loop_n: bool, loop_m: bool) -> GraphRaw;
    fn emptyGraph(nodes: i32) -> GraphRaw;
    fn cubeGraph(n: i32) -> GraphRaw;
    fn completeKPartiteGraph() -> GraphRaw;
    fn completeGraph(n: i32) -> GraphRaw;
    fn completeBipartiteGraph(n: i32, m: i32) -> GraphRaw;
    fn circulantGraph(n: i32) -> GraphRaw;
}

/// **Parameters:**
///
/// `n` – is the number of nodes of the tree.
///
/// `children` – is the number of children per node. root has index 0, the next level has indizes 1...children, the children of node 1 have indizes children+1...2*children, etc. if number of nodes does not allow a regular node, the "last" node will have fewer children.
pub fn regular_tree(n: i32, children: i32) -> GraphRaw {
    unsafe { regularTree(n, children) }
}

/// Generates a cycle on n sequential nodes, where any two nodes whose distance is at most k / 2 are connected by an additional edge.
///
/// **Parameters:**
///
/// `n` – is the number of nodes in the graph.
///
/// `k` – is the degree of each node.
pub fn regular_lattice(n: i32, k: i32) -> GraphRaw {
    unsafe { regularLatticeGraph(n, k) }
}

/// Creates an outer cycle of nodes 1, ..., n, each of which has a direct neighbor (a corresponding inner node). For two outer nodes i, j, there is an edge between their corresponding inner nodes if the absolute difference of i and j equals the jump length m. If no values for n or m are given, assume the standard Petersen graph of 5 nodes and a jump length of 2.
///
/// **Parameters:**
///
/// `n` – is the number of nodes on the outer cycle.
///
/// `m` – is the length of jumps for the inner part.
pub fn petersen(n: i32, m: i32) -> GraphRaw {
    unsafe { petersenGraph(n, m) }
}

/// **Parameters:**
///
/// `n` – is the number of nodes on first axis.
///
/// `m` – is the number of nodes on second axis.
///
/// `loop_n` – if the grid is cyclic on first axis
///
/// `loop_m` – if the grid is cyclic on second axis
pub fn grid(n: i32, m: i32, loop_n: bool, loop_m: bool) -> GraphRaw {
    unsafe { gridGraph(n, m, loop_n, loop_m) }
}

/// **Parameters:**
///
/// `nodes` – is the number of nodes of the generated graph.
pub fn empty(nodes: i32) -> GraphRaw {
    unsafe { emptyGraph(nodes) }
}

/// **Parameters:**
///
/// `n` – is the number of the cube's dimensions (n>=0).
pub fn cube(n: i32) -> GraphRaw {
    unsafe { cubeGraph(n) }
}

/// The returned graph is directed acyclic.
///
/// **Parameters:**
///
/// `signature` – contains the positive values k1, k2, ..., kn.
pub fn complete_k_partite() -> GraphRaw {
    unsafe { completeKPartiteGraph() }
}

/// The returned graph is directed acyclic.
///
/// **Parameters:**
///
/// `n` – is the number of nodes of the generated graph.
pub fn complete(n: i32) -> GraphRaw {
    unsafe { completeGraph(n) }
}

/// The returned graph is directed acyclic.
///
/// **Parameters:**
///
/// `n` – is the number of nodes of the first partition set.
///
/// `m` – is the number of nodes of the second partition set.
pub fn complete_bipartite(n: i32, m: i32) -> GraphRaw {
    unsafe { completeBipartiteGraph(n, m) }
}

/// Generates a simple, undirected graph on
///
/// **Parameters:**
///
/// `n` – is the number of nodes of the generated graph.
///
/// `jumps` – is the array of distances for edges to be created. ogdf::Graph G; ogdf::circulantGraph(G, 11, ogdf::Array<int>({1,2,4}));
pub fn circulant(n: i32) -> GraphRaw {
    unsafe { circulantGraph(n) }
}
