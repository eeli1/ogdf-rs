#include <ogdf/basic/Graph_d.h>
#include "graph.h"

void freeGraphRaw(GraphRaw &gr)
{
    ogdf::Graph *g = (ogdf::Graph *)gr.raw;
    delete g;
    free(g);
}