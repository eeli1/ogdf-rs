#pragma once

typedef enum
{
    Graph,
    Error
} GraphType;

typedef struct GraphRaw
{
    GraphType graph_type;
    void *raw;
};
