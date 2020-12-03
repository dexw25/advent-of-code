//
// Created by Derek Witt on 12/2/20.
//
#include "day_03.hpp"
#include "day_03_input.hpp"

#include <array>
#include <iostream>

constexpr const std::array<const char*, 11> in = {{
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
}};

int main() {
    static_assert(day_03_1(in) == 7);
    static_assert(day_03_1(day_03_data) == 189);
    static_assert(day_03_2(in) == 336);
    static_assert(day_03_2(day_03_data) == 1718180100);
}