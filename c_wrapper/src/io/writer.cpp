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
    std::ostringstream os;
    ogdf::Graph G(*((ogdf::Graph *)gr.raw));
    if (!ogdf::GraphIO::writeGraphML(G, os))
        return nullptr;
    std::string str = os.str();
    const char *ptr = str.c_str();
    return ptr;
}