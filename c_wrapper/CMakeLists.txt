cmake_minimum_required(VERSION 3.1)
project(OGDF-SYS CXX)

add_library(c_wrapper STATIC test.cpp graph.cpp)

add_subdirectory(ogdf)
include_directories(c_wrapper ogdf/include)
#link_directories(../ogdf)

# set(CMAKE_CXX_FLAGS "-L../ogdf")

# target_link_libraries(c_wrapper PRIVATE  libOGDF.a)
# target_link_libraries(c_wrapper PRIVATE  libCOIN.a) 



# include(ExternalProject)
# ExternalProject_Add(ogdf
#   GIT_REPOSITORY    https://github.com/eeli1/ogdf
#   GIT_TAG           master
#   SOURCE_DIR        "${CMAKE_BINARY_DIR}/src"
#   BINARY_DIR        "${CMAKE_BINARY_DIR}"
#   CONFIGURE_COMMAND ""
#   BUILD_COMMAND     ""
#   INSTALL_COMMAND   ""
#   TEST_COMMAND      ""
# )

install (TARGETS c_wrapper DESTINATION .)