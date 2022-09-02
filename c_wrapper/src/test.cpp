#include "test.h"

#include <ogdf/basic/internal/version.h>
#include <ogdf/basic/graph_generators.h>
#include <ogdf/planarity/SubgraphPlanarizer.h>
#include <ogdf/planarity/PlanarSubgraphFast.h>
#include <ogdf/planarity/VariableEmbeddingInserter.h>
#include <ogdf/fileformats/GraphIO.h>

#include <iostream>

void test()
{
    std::cout << "test" << std::endl;
}

int double_input(int input)
{
    return input * 2;
}

const char *say_hello()
{
    return OGDF_VERSION;
}

void print_str_aaa(const char *str)
{
    std::cout << "ptr: " << &str << "\tvalue: " << str << std::endl;
}