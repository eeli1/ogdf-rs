#pragma once
#include "../graph.h"

extern "C" GraphRaw circulantGraph(int n);
extern "C" GraphRaw completeBipartiteGraph(int n, int m);
extern "C" GraphRaw completeGraph(int n);
extern "C" GraphRaw completeKPartiteGraph();
extern "C" GraphRaw cubeGraph(int n);
extern "C" GraphRaw emptyGraph(int nodes);
extern "C" GraphRaw gridGraph(int n, int m, bool loopN, bool loopM);
extern "C" GraphRaw petersenGraph(int n, int m);
extern "C" GraphRaw regularLatticeGraph(int n, int k);
extern "C" GraphRaw regularTree(int n, int children);
