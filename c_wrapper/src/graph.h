#pragma once

typedef struct GraphRaw
{

    typedef enum
    {
        Graph,
        Error,
        Biconnec,
        GeometricCube,
        Digraph,
        Hierarchy,
        PlanarBiconnectedDigraph,
        DAG,
        Simple,
        Tree,
        Triconnected,
        Waxman,
        WattsStrogatz,
        Bipartite,
        Grid,
    } Type;

    Type type;
    void *raw;
};
