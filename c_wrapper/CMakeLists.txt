cmake_minimum_required(VERSION 3.1)
project(OGDF-SYS CXX)

add_library(c_wrapper STATIC test.cpp graph.cpp)

add_subdirectory(ogdf)
include_directories(c_wrapper ogdf/include)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Logdf")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Logdf")

target_link_libraries(c_wrapper libOGDF.a)
target_link_libraries(c_wrapper libCOIN.a) 

install (TARGETS c_wrapper DESTINATION .)