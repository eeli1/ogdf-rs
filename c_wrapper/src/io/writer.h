#pragma once
#include "../graph.h"

extern "C" const char *writeChaco(GraphRaw gr);
extern "C" const char *writeDL(GraphRaw gr);
extern "C" const char *writeDMF(GraphRaw gr);
extern "C" const char *writeDOT(GraphRaw gr);
extern "C" const char *writeGDF(GraphRaw gr);
extern "C" const char *writeGEXF(GraphRaw gr);
extern "C" const char *writeGML(GraphRaw gr);
extern "C" const char *writeGraphML(GraphRaw gr);
extern "C" const char *writeLEDA(GraphRaw gr);
extern "C" const char *writeMatrixMarket(GraphRaw gr);
extern "C" const char *writePMDissGraph(GraphRaw gr);
extern "C" const char *writeRome(GraphRaw gr);
extern "C" const char *writeRudy(GraphRaw gr);
extern "C" const char *writeSTP(GraphRaw gr);
extern "C" const char *writeTLP(GraphRaw gr);
extern "C" const char *writeTsplibXml(GraphRaw gr);