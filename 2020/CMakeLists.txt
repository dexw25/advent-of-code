cmake_minimum_required(VERSION 3.17)
set(CMAKE_CXX_STANDARD 20)
set(CXX_EXTENSIONS ON)

project(AOC_2020)

# Global
add_compile_options(-Wall -Wextra -Wshadow -Wnon-virtual-dtor -Wpedantic)

# External libs
add_subdirectory(extern/fmt)

add_subdirectory(day_01)

add_executable(run_all main.cpp)

target_link_libraries(run_all fmt::fmt day_01)

enable_testing()
add_subdirectory(tests)
