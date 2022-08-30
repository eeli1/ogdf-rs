#include <ogdf/fileformats/GraphIO.h>
#include "writer.h"

const char *writeChaco(GraphRaw gr)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!ogdf::GraphIO::writeChaco(g, os))
        return nullptr;
    static std::string str = os.str();
    const char *c = str.c_str();
    return c;
}
const char *writeDL(GraphRaw gr)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!ogdf::GraphIO::writeDL(g, os))
        return nullptr;
    static std::string str = os.str();
    const char *c = str.c_str();
    return c;
}
const char *writeDOT(GraphRaw gr)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!ogdf::GraphIO::writeDOT(g, os))
        return nullptr;
    static std::string str = os.str();
    const char *c = str.c_str();
    return c;
}
const char *writeGDF(GraphRaw gr)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!ogdf::GraphIO::writeGDF(g, os))
        return nullptr;
    static std::string str = os.str();
    const char *c = str.c_str();
    return c;
}
const char *writeGEXF(GraphRaw gr)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!ogdf::GraphIO::writeGEXF(g, os))
        return nullptr;
    static std::string str = os.str();
    const char *c = str.c_str();
    return c;
}
const char *writeGML(GraphRaw gr)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!ogdf::GraphIO::writeGML(g, os))
        return nullptr;
    static std::string str = os.str();
    const char *c = str.c_str();
    return c;
}
const char *writeGraphML(GraphRaw gr)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!ogdf::GraphIO::writeGraphML(g, os))
        return nullptr;
    static std::string str = os.str();
    const char *c = str.c_str();
    return c;
}
const char *writeLEDA(GraphRaw gr)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!ogdf::GraphIO::writeLEDA(g, os))
        return nullptr;
    static std::string str = os.str();
    const char *c = str.c_str();
    return c;
}
const char *writePMDissGraph(GraphRaw gr)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!ogdf::GraphIO::writePMDissGraph(g, os))
        return nullptr;
    static std::string str = os.str();
    const char *c = str.c_str();
    return c;
}
const char *writeRome(GraphRaw gr)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!ogdf::GraphIO::writeRome(g, os))
        return nullptr;
    static std::string str = os.str();
    const char *c = str.c_str();
    return c;
}
const char *writeTLP(GraphRaw gr)
{
    ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    if (!ogdf::GraphIO::writeTLP(g, os))
        return nullptr;
    static std::string str = os.str();
    const char *c = str.c_str();
    return c;
}