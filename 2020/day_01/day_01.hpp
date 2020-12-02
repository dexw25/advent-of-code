//
// Created by Derek Witt on 12/2/20.
//

#ifndef ADVENT_OF_CODE_DAY_01_H
#define ADVENT_OF_CODE_DAY_01_H


template<typename T>
constexpr int day_01_1(const T &input) {
    for (auto i: input) {
        for (auto j: input) {
            if (i + j == 2020) {
                return i * j;
            }
        }
    }
    return 0;
}

template<typename T>
constexpr int day_01_2(const T &input) {
    for (auto i: input) {
        for (auto j: input) {
            for (auto h: input) {
                if (i + j + h == 2020) {
                    return i * j * h;
                }
            }
        }
    }
    return 0;
}

#endif //ADVENT_OF_CODE_DAY_01_H
