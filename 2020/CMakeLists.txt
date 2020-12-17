cmake_minimum_required(VERSION 3.17)
set(CMAKE_CXX_STANDARD 20)
set(CXX_EXTENSIONS ON)

project(AOC_2020)
set(CONAN_DISABLE_CHECK_COMPILER)
include(conan.cmake)
conan_cmake_run(REQUIRES fmt/7.1.3
        BASIC_SETUP CMAKE_TARGETS
        BUILD missing)

# Global
add_compile_options(-Wall -Wextra -Wshadow -Wnon-virtual-dtor -Wpedantic)

add_subdirectory(day_01)
add_subdirectory(day_02)
add_subdirectory(day_03)
add_subdirectory(day_04)
add_subdirectory(day_05)
add_subdirectory(day_06)
add_subdirectory(day_07)
add_subdirectory(day_08)
add_subdirectory(day_09)
add_subdirectory(day_10)

add_subdirectory(gconsole)

# Cross deps
target_link_libraries(day_04 INTERFACE day_02)
target_link_libraries(day_08 INTERFACE gconsole)

add_executable(run_all main.cpp)

target_link_libraries(run_all CONAN_PKG::fmt day_01 day_02 day_03 day_04 day_05 day_06 day_07 day_08 day_09 day_10)

enable_testing()
add_subdirectory(tests)
