#pragma once
#include "../graph.h"

extern "C" GraphRaw randomBiconnectedGraph(int n, int m);
extern "C" GraphRaw randomUpwardPlanarBiconnectedDigraph(int n, int m);
extern "C" GraphRaw randomWattsStrogatzGraph(int n, int k, double probability);
extern "C" GraphRaw randomWaxmanGraph(int nodes, double alpha, double beta, double width, double height);
extern "C" GraphRaw randomTriconnectedGraph(int n, double p1, double p2);
extern "C" GraphRaw randomTree(int n, int maxDeg, int maxWidth);
extern "C" GraphRaw randomSimpleGraphByProbability(int n, double pEdge);
extern "C" GraphRaw randomSimpleGraph(int n, int m);
extern "C" GraphRaw randomSimpleConnectedGraph(int n, int m);
extern "C" GraphRaw randomSeriesParallelDAG(int edges, double p, double flt);
extern "C" GraphRaw randomRegularGraph(int n, int d);
extern "C" GraphRaw randomPlanarBiconnectedDigraph(int n, int m, double p, bool multiEdges);
extern "C" GraphRaw randomHierarchy(int n, int m, bool planar, bool singleSource, bool longEdges);
extern "C" GraphRaw randomGraph(int n, int m);
extern "C" GraphRaw randomGeometricCubeGraph(int nodes, double threshold, int dimension);
extern "C" GraphRaw randomDigraph(int n, double p);
extern "C" void setSeed(int val);