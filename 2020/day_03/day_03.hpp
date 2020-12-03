//
// Created by Derek Witt on 12/2/20.
//

#ifndef AOC_2020_DAY_03_HPP
#define AOC_2020_DAY_03_HPP

#include <algorithm>
#include <iostream>
#include <iostream>

// Hardcode a slope of 3,1
template<size_t N>
constexpr int trees_in_path(const std::array<const char*, N> &input, const int run, const int rise) {
    int trees = 0;
    int j = 0, i = 0;
    int length = 0;
    auto c = &input[0][0];
    while (*c++) length++; // Calculate width for modulo operator

    // For each line
    for (auto it: input) {
        // Only evaluate points that match the slope
        if (i % rise == 0) {
            if (it[j % length] == '#') trees++; // Determine if a tree is on our tile
            j += run; // Only bump the run if row matches
        }
        i++; // line counter
    }
    return trees;
}

template<size_t N>
constexpr int day_03_1(const std::array<const char*, N> &input) {
    return trees_in_path(input, 3, 1);
}

template<size_t N>
constexpr int day_03_2(const std::array<const char*, N> &input) {
    int a = trees_in_path(input, 1, 1);
    int b = trees_in_path(input, 3, 1);
    int c = trees_in_path(input, 5, 1);
    int d = trees_in_path(input, 7, 1);
    int e = trees_in_path(input, 1, 2);
    return a * b * c * d * e;
}

#endif //AOC_2020_DAY_03_HPP
