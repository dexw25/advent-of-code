cmake_minimum_required(VERSION 3.17)

add_library(day_02 STATIC day_02.cpp)
target_include_directories(day_02 PUBLIC ${CMAKE_CURRENT_LIST_DIR})