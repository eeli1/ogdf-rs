#include "generate.h"
#include <ogdf/basic/graph_generators.h>

GraphRaw circulantGraph(int n)
{
    ogdf::Array<int> jumps;
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::circulantGraph(*g, n, jumps);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Graph;
    return gr;
}

GraphRaw completeBipartiteGraph(int n, int m)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::completeBipartiteGraph(*g, n, m);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Bipartite;
    return gr;
}

GraphRaw completeGraph(int n)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::completeGraph(*g, n);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Graph;
    return gr;
}

GraphRaw completeKPartiteGraph()
{
    ogdf::Array<int> signature;
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::completeKPartiteGraph(*g, signature);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Graph;
    return gr;
}

GraphRaw cubeGraph(int n)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::cubeGraph(*g, n);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Graph;
    return gr;
}

GraphRaw emptyGraph(int nodes)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::emptyGraph(*g, nodes);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Graph;
    return gr;
}

GraphRaw gridGraph(int n, int m, bool loopN, bool loopM)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::gridGraph(*g, n, m, loopN, loopM);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Grid;
    return gr;
}

GraphRaw petersenGraph(int n, int m)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::petersenGraph(*g, n, m);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Graph;
    return gr;
}

GraphRaw regularLatticeGraph(int n, int k)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::regularLatticeGraph(*g, n, k);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Graph;
    return gr;
}

GraphRaw regularTree(int n, int children)
{
    ogdf::Graph *g = new ogdf::Graph();
    ogdf::regularTree(*g, n, children);
    GraphRaw gr;
    gr.raw = (void *)g;
    gr.type = GraphRaw::Type::Tree;
    return gr;
}
