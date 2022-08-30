#include <ogdf/fileformats/GraphIO.h>
#include "reader.h"

GraphRaw readAbstractGraph(const char *str, ogdf::GraphIO::ReaderFunc func)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!func(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}

GraphRaw readChaco(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readChaco(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readDL(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readDL(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readDMF(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readDMF(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readDOT(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readDOT(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readGDF(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readGDF(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readGEXF(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readGEXF(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readGML(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readGML(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readGraphML(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readGraphML(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readLEDA(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readLEDA(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readMatrixMarket(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readMatrixMarket(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readPMDissGraph(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readPMDissGraph(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readRome(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readRome(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readRudy(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readRudy(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readSTP(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readSTP(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readTLP(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readTLP(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}
GraphRaw readTsplibXml(const char *str)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    static ogdf::Graph g;
    GraphRaw gr;
    gr.raw = (void *)&g;
    if (!ogdf::GraphIO::readTsplibXml(g, is))
        gr.graph_type = GraphType::Error;
    else
        gr.graph_type = GraphType::Graph;

    return gr;
}

/*
GraphRaw readBENCH(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readBENCH);
}
GraphRaw readChallengeGraph(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readChallengeGraph);
}
GraphRaw readDigraph6(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readDigraph6);
}
GraphRaw readGraph6(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readGraph6);
}
GraphRaw readPLA(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readPLA);
}
GraphRaw readSparse6(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readSparse6);
}
GraphRaw readEdgeListSubgraph(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readEdgeListSubgraph);
}
*/