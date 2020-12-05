//
// Created by Derek Witt on 12/5/20.
//

#ifndef AOC_2020_DAY_05_H
#define AOC_2020_DAY_05_H

#include <string_view>
#include <tuple>
#include <array>
#include <limits>

using namespace std;

int constexpr char_to_bit(const char c) {
    if (c == 'B' || c == 'R') return 1;
    else return 0;
}

// 7 F or B (front or back), 3 L or R, treat as bits in a number to map seat row and column
 constexpr int parse_pass(const string_view seat) {
    int acc = 0;
    assert(seat.size() == 10);

    // Pack it all into one accumulator for simplicity
    for (auto c: seat) {
        acc += char_to_bit(c);
        acc <<= 1;
    }

    // Acc is overshifted by one, shift back one
    acc >>= 1;

    // ID is value of accumulator
    return acc;
}

// Just parse the list and get the max
template <size_t N>
constexpr int day_05_1 (const array<const string_view, N> &tickets) {
    int highest_id = 0;

    // Parse all ids and save highest
    for (auto s: tickets) {
        auto id = parse_pass(s);
        if (id > highest_id) highest_id = id;
    }

    return highest_id;
}

// Find the gap in the list
template <size_t N>
constexpr int day_05_2 (const array<const string_view, N> &tickets) {
    int highest_id = 0;
    int lowest_id = numeric_limits<int>::max();
    int acc = 0;
    int full_acc = 0;
    int ret;

    // Parse all ids, dave highest and lowest, and add ID to acc
    for (auto s: tickets) {
        auto id = parse_pass(s);
        if (id > highest_id) highest_id = id;
        if (id < lowest_id) lowest_id = id;
        acc += id;
    }

    // All slots are filled. Generate a sequence from 0 to min_id and add to acc so we have the sum of 0-max - missing ids
    for(int i=0; i < lowest_id; i++) {
        acc += i;
    }

    // Generate what the sequence would be if it was continuous (could sub formula here if compile times are too long)
    for(int i=0; i <= highest_id; i++) {
        full_acc += i;
    }

    // With only one is missing, it must be our seat
    ret = full_acc - acc;

    return ret;

}



#endif //AOC_2020_DAY_05_H
