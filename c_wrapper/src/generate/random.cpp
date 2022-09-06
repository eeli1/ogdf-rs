#include <ogdf/basic/graph_generators.h>
#include "random.h"


GraphRaw randomBiconnectedGraph(int n, int m)
{

    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomBiconnectedGraph(*g, n, m);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Biconnec;
    return gr;
}


GraphRaw randomUpwardPlanarBiconnectedDigraph(int n, int m)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomUpwardPlanarBiconnectedDigraph(*g, n, m);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::PlanarBiconnectedDigraph;
    return gr;
}

GraphRaw randomWattsStrogatzGraph(int n, int k, double probability)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomWattsStrogatzGraph(*g, n, k, probability);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Triconnected;
    return gr;
}

GraphRaw randomWaxmanGraph(int nodes, double alpha, double beta, double width, double height)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomWaxmanGraph(*g, nodes, alpha, beta, width, height);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Waxman;
    return gr;
}

GraphRaw randomTriconnectedGraph(int n, double p1, double p2)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomTriconnectedGraph(*g, n, p1, p2);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Triconnected;
    return gr;
}


GraphRaw randomTree(int n, int maxDeg, int maxWidth)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomTree(*g, n, maxDeg, maxWidth);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Tree;
    return gr;
}


GraphRaw randomSimpleGraphByProbability(int n, double pEdge)
{
    ogdf::Graph *g = new ogdf::Graph();
    GraphRaw gr;
    if (!ogdf::randomSimpleGraphByProbability(*g, n, pEdge))
        gr.type = GraphRaw::Type::Error;
    else
        gr.type = GraphRaw::Type::Simple;
    gr.raw = (void *)g;
    return gr;
}

GraphRaw randomSimpleGraph(int n, int m)
{
    ogdf::Graph *g = new ogdf::Graph();
    GraphRaw gr;
    if (!ogdf::randomSimpleGraph(*g, n, m))
        gr.type = GraphRaw::Type::Error;
    else
        gr.type = GraphRaw::Type::Simple;
    gr.raw = (void *)g;
    return gr;
}

GraphRaw randomSimpleConnectedGraph(int n, int m)
{
    ogdf::Graph *g = new ogdf::Graph();
    GraphRaw gr;
    if (!ogdf::randomSimpleConnectedGraph(*g, n, m))
        gr.type = GraphRaw::Type::Error;
    else
        gr.type = GraphRaw::Type::Simple;
    gr.raw = (void *)g;
    return gr;
}

GraphRaw randomSeriesParallelDAG(int edges, double p, double flt)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomSeriesParallelDAG(*g, edges, p, flt);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::DAG;
    return gr;
}


GraphRaw randomRegularGraph(int n, int d)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomRegularGraph(*g, n, d);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Graph;
    return gr;
}

GraphRaw randomPlanarBiconnectedDigraph(int n, int m, double p, bool multiEdges)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomPlanarBiconnectedDigraph(*g, n, m, p, multiEdges);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::PlanarBiconnectedDigraph;
    return gr;
}

GraphRaw randomHierarchy(int n, int m, bool planar, bool singleSource, bool longEdges)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomHierarchy(*g, n, m, planar, singleSource, longEdges);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Hierarchy;
    return gr;
}

GraphRaw randomGraph(int n, int m)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomGraph(*g, n, m);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Graph;
    return gr;
}

GraphRaw randomGeometricCubeGraph(int nodes, double threshold, int dimension)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomGeometricCubeGraph(*g, nodes, threshold, dimension);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::GeometricCube;
    return gr;
}

GraphRaw randomDigraph(int n, double p)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::randomDigraph(*g, n, p);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Digraph;
    return gr;
}


void setSeed(int val){
    ogdf::setSeed(val);
}


/// Implements the algorithm described in: The average distance in a random graph with given expected degrees Fang Chung and Linyuan Lu http://www.math.ucsd.edu/~fan/wp/aveflong.pdf Given an expected degree distribution of length n:
///
/// `expectedDegreeDistribution` – is a list of expected degrees, or weights, for the individual nodes. Its length defines the number of nodes n.
// GraphRaw randomChungLuGraph(int *expectedDegreeDistribution, size_t size)
// {
//    std::vector<int> vec;
// std::copy(vec.begin(), vec.end(), expectedDegreeDistribution, size);

//     ogdf::Graph g;
//     ogdf::Array<int> arr;
//     arr.fill(0, size - 1, *expectedDegreeDistribution);
//     ogdf::randomChungLuGraph(g, arr);
// }

// ogdf::randomClusterGraph
// ogdf::randomClusterPlanarGraph()