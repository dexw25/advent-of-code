//
// Created by Derek Witt on 12/2/20.
//

#ifndef ADVENT_OF_CODE_DAY_01_H
#define ADVENT_OF_CODE_DAY_01_H

#include <iostream>
#include <algorithm>

template<typename T>
constexpr int day_01_1(const T &input) {
    // Using iterators allows some pruning of the input dataset, at the cost of slightly less concise code
    for (auto it = input.begin(); it < input.end() - 1; it++) {
        // Since we only care for combinations, start at the iterator after the current one
        for (auto jt = it + 1; jt < input.end(); jt++) {
            if (*it + *jt == 2020) {
                // This should only happen once if the dataset is clean, assume first hit is good
                return *it * *jt;
            }
        }
    }

    return 0;
}

// This exhaustive search of all combinations is optimal, every case tested is unique, but it still takes seconds to build in constexpr use cases
template<typename T>
constexpr int day_01_2(const T &input) {
    auto end = input.end();
    // Search the tree of all possible combinations, use iterators and restricted for loops to only get combinations
    // This has been mathematically verified to generate the correct number of combinations on a sample data set
    for (auto it = input.begin(); end - 1; it++) {
        // Exclude anything already done by outer loop
        for (auto jt = it + 1; jt < end - 1; jt++) {
            // Exclude outer loop but include final number
            for (auto kt = jt + 1; kt < end; kt++) {
                if ((*it + *jt + *kt) == 2020) {
                    return *it * *jt * *kt; // Cumulative product
                }
            }
        }
    }
    return 0;
}

#endif //ADVENT_OF_CODE_DAY_01_H
