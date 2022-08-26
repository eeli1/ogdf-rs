#include "graph.h"

void test()
{
	Graph G;
	randomSimpleGraph(G, 100, 150);

	SubgraphPlanarizer SP;
	SP.setSubgraph(new PlanarSubgraphFast<int>);
	SP.setInserter(new VariableEmbeddingInserter);
		
	int crossNum;
	PlanRep PR(G);
	SP.call(PR, 0, crossNum);
		
	std::cout << crossNum << " crossings" << std::endl;
	// GraphIO::write(PR, "output-plan.gml", GraphIO::writeGML);

}