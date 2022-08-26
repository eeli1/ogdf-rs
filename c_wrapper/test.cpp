#include <ogdf/basic/internal/version.h>

extern "C"
int double_input(int input) {
    return input * 2;
}

extern "C"
const char* say_hello(){
    return OGDF_VERSION;
}