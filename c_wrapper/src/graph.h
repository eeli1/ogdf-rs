#pragma once

typedef struct GraphRaw
{

    typedef enum
    {
        Graph = 0,
        Error = 1,
        Biconnec = 2,
    } Type;

    Type type;
    void *raw;
};
