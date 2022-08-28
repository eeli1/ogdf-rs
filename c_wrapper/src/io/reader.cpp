#include <ogdf/fileformats/GraphIO.h>
#include "reader.h"

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
