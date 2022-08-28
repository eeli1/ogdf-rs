#include <ogdf/fileformats/GraphIO.h>
#include "writer.h"

const char *writeChaco(GraphRaw gr)
{
    std::ostringstream os;
    ogdf::Graph G(*((ogdf::Graph *)gr.raw));
    if (!ogdf::GraphIO::writeChaco(G, os))
        return nullptr;
    std::string str = os.str();
    const char *ptr = str.c_str();
    return ptr;
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