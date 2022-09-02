#include <ogdf/basic/graph_generators.h>
#include "random.h"

/// `n` – is the number of nodes of the generated graph.
///
/// `m` – is the number of edges of the generated graph.
GraphRaw randomBiconnectedGraph(int n, int m)
{
    static ogdf::Graph g;
    ogdf::randomBiconnectedGraph(g, n, m);
    // ogdf::randomClusterGraph()
    // ogdf::randomClusterPlanarGraph
    // ogdf::randomDigraph
    // ogdf::randomGeometricCubeGraph
    // ogdf::randomGraph
    // ogdf::randomHierarchy
    // ogdf::randomPlanarBiconnectedDigraph
    // ogdf::randomRegularGraph
    // ogdf::randomSeriesParallelDAG
    // ogdf::randomSimpleConnectedGraph
    // ogdf::randomSimpleGraph
    // ogdf::randomSimpleGraphByProbability
    // ogdf::randomTree
    // ogdf::randomTriconnectedGraph
    // ogdf::randomUpwardPlanarBiconnectedDigraph
    // ogdf::randomUpwardPlanarBiconnectedDigraph
    // ogdf::randomWattsStrogatzGraph
    // ogdf::randomWaxmanGraph

    GraphRaw gr;
    return gr;
    // return GraphRaw((void *)&g, GraphRaw::Type::Biconnec);
}

/// Implements the algorithm described in: The average distance in a random graph with given expected degrees Fang Chung and Linyuan Lu http://www.math.ucsd.edu/~fan/wp/aveflong.pdf Given an expected degree distribution of length n:
///
/// `expectedDegreeDistribution` – is a list of expected degrees, or weights, for the individual nodes. Its length defines the number of nodes n.
GraphRaw randomChungLuGraph(int *expectedDegreeDistribution, size_t size)
{
    std::vector<int> vec;
    ;
    // std::copy(il.begin(), il.end(), expectedDegreeDistribution, size);

    static ogdf::Graph g;
    ogdf::Array<int> arr;
    arr.fill(0, size - 1, *expectedDegreeDistribution);
    ogdf::randomChungLuGraph(g, arr);
}