#include <ogdf/fileformats/GraphIO.h>
#include "reader.h"

// this is only for iternal use
GraphRaw readAbstractGraph(const char *str, ogdf::GraphIO::ReaderFunc func)
{
    std::string s = str;
    std::stringbuf buf(s);
    std::istream is(&buf);
    ogdf::Graph *g = new ogdf::Graph();
    GraphRaw gr;
    gr.raw = (void *)g;
    if (!func(*g, is))
        gr.type = GraphRaw::Type::Error;
    else
        gr.type = GraphRaw::Type::Graph;
    return gr;
}

GraphRaw readChaco(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readChaco);
}
GraphRaw readDL(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readDL);
}
GraphRaw readDMF(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readDMF);
}
GraphRaw readDOT(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readDOT);
}
GraphRaw readGDF(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readGDF);
}
GraphRaw readGEXF(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readGEXF);
}
GraphRaw readGML(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readGML);
}
GraphRaw readGraphML(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readGraphML);
}
GraphRaw readLEDA(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readLEDA);
}
GraphRaw readMatrixMarket(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readMatrixMarket);
}
GraphRaw readPMDissGraph(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readPMDissGraph);
}
GraphRaw readRome(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readRome);
}
GraphRaw readRudy(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readRudy);
}
GraphRaw readSTP(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readSTP);
}
GraphRaw readTLP(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readTLP);
}
GraphRaw readTsplibXml(const char *str)
{
    return readAbstractGraph(str, ogdf::GraphIO::readTsplibXml);
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