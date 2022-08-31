#include <ogdf/fileformats/GraphIO.h>
#include "writer.h"

const char *writeAbstractGraph(GraphRaw gr, ogdf::GraphIO::WriterFunc func)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!func(g, os))
        return nullptr;
    std::string str = os.str();
    char *ptr = new char[str.length() + 1];
    strcpy(ptr, str.c_str());
    return ptr;
}

const char *writeChaco(GraphRaw gr)
{
    return writeAbstractGraph(gr, ogdf::GraphIO::writeChaco);
}
const char *writeDL(GraphRaw gr)
{
    return writeAbstractGraph(gr, ogdf::GraphIO::writeDL);
}
const char *writeDOT(GraphRaw gr)
{
    return writeAbstractGraph(gr, ogdf::GraphIO::writeDOT);
}
const char *writeGDF(GraphRaw gr)
{
    return writeAbstractGraph(gr, ogdf::GraphIO::writeGDF);
}
const char *writeGEXF(GraphRaw gr)
{
    return writeAbstractGraph(gr, ogdf::GraphIO::writeGEXF);
}
const char *writeGML(GraphRaw gr)
{
    return writeAbstractGraph(gr, ogdf::GraphIO::writeGML);
}
const char *writeGraphML(GraphRaw gr)
{
    return writeAbstractGraph(gr, ogdf::GraphIO::writeGraphML);
}
const char *writeLEDA(GraphRaw gr)
{
    return writeAbstractGraph(gr, ogdf::GraphIO::writeLEDA);
}
const char *writePMDissGraph(GraphRaw gr)
{
    return writeAbstractGraph(gr, ogdf::GraphIO::writePMDissGraph);
}
const char *writeRome(GraphRaw gr)
{
    return writeAbstractGraph(gr, ogdf::GraphIO::writeRome);
}
const char *writeTLP(GraphRaw gr)
{
    return writeAbstractGraph(gr, ogdf::GraphIO::writeTLP);
}
