cmake_minimum_required(VERSION 3.1)
project(OGDF-SYS CXX)

#add_library(c_wrapper STATIC test.cpp graph.cpp)
add_executable(c_wrapper test.cpp graph.cpp main.cpp)

add_subdirectory(ogdf)
include_directories(c_wrapper ogdf/include)
#link_directories(../ogdf)

set(CMAKE_CXX_FLAGS "-Logdf")

target_link_libraries(c_wrapper libOGDF.a)
target_link_libraries(c_wrapper libCOIN.a) 

set(THREADS_PREFER_PTHREAD_FLAG ON)
find_package(Threads REQUIRED)
target_link_libraries(c_wrapper Threads::Threads )

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