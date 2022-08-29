#pragma once
#include "../graph.h"

extern "C" GraphRaw readChaco(const char *str);
extern "C" GraphRaw readDL(const char *str);
extern "C" GraphRaw readDMF(const char *str);
extern "C" GraphRaw readDOT(const char *str);
extern "C" GraphRaw readGDF(const char *str);
extern "C" GraphRaw readGEXF(const char *str);
extern "C" GraphRaw readGML(const char *str);
extern "C" GraphRaw readGraphML(const char *str);
extern "C" GraphRaw readLEDA(const char *str);
extern "C" GraphRaw readMatrixMarket(const char *str);
extern "C" GraphRaw readPMDissGraph(const char *str);
extern "C" GraphRaw readRome(const char *str);
extern "C" GraphRaw readRudy(const char *str);
extern "C" GraphRaw readSTP(const char *str);
extern "C" GraphRaw readTLP(const char *str);
extern "C" GraphRaw readTsplibXml(const char *str);
