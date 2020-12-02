cmake_minimum_required(VERSION 3.17)

add_library(day_01 INTERFACE)
target_include_directories(day_01 INTERFACE ${CMAKE_CURRENT_LIST_DIR})

target_compile_options(day_01 INTERFACE -fconstexpr-steps=4000000)