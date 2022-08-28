// only used for debugging

#include <iostream>
#include "io/writer.h"
#include "io/reader.h"
#include "graph.h"
#include <ogdf/fileformats/GraphIO.h>

GraphRaw read()
{
    const char *graph_ml = "<?xml version=\"1.0\"?>\
<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns&#10;http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\">\
    <graph id=\"G\" edgedefault=\"directed\">\
        <node id=\"0\" />\
        <node id=\"1\" />\
        <edge id=\"0\" source=\"0\" target=\"1\" />\
    </graph>\
</graphml>";
    std::string s = graph_ml;
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

int main()
{
    const char *graph_ml = "<?xml version=\"1.0\"?>\
<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns&#10;http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\">\
    <graph id=\"G\" edgedefault=\"directed\">\
        <node id=\"0\" />\
        <node id=\"1\" />\
        <edge id=\"0\" source=\"0\" target=\"1\" />\
    </graph>\
</graphml>";

    // GraphRaw gr = read();// readGraphML(graph_ml);

    ogdf::Graph g;
    std::string s = graph_ml;
    std::stringbuf buf(s);
    std::istream is(&buf);

    ogdf::GraphIO::readGraphML(g, is);

    // ogdf::Graph g = *((ogdf::Graph *)gr.raw);
    std::ostringstream os;
    ogdf::GraphIO::writeGraphML(g, os);
    static std::string str = os.str();
    const char *c = str.c_str();
    std::cout << c << std::endl;
}