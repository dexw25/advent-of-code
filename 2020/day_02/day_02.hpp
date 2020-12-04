//
// Created by Derek Witt on 12/2/20.
//

#ifndef AOC_2020_DAY_02_HPP
#define AOC_2020_DAY_02_HPP
#include <algorithm>
#include <iostream>

// Some cstring functions do not have constexpr implementations, so I wrote them here
//  I know that constexpr is a little silly for this but I thought it was a neat challenge to do it this way
constexpr bool is_digit(const char c) {
    return (c >= '0' && c <= '9');
}

// Special case for hex digits
constexpr bool is_hexdigit(const char c) {
    return ((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'));
}
// Convert ints with some basic recursion
constexpr int stoi_impl(const char *str, int val = 0) {
    if (*str) {
        if (is_digit(*str)) {
            return stoi_impl(str + 1, *str - '0' + val * 10);
        }
    }
    return val;
}

// Seek to a character in a string
constexpr const char *strchr(const char *str, const char c) {
    while(*str != c && *str){
        str++;
    }
    return str;
}

// Count valid passwords in input (input should be a sequence of cstrings)
template <typename T>
constexpr int day_02_1(T input){
    int valid = 0; // Count valid passwords

    // A spec for password rules is NN-NN X: xxxxxxxxxx....xxxxxxxxxx
    // NN is 1-2 digit int, min for total X's in input, second NN is max
    std::for_each(input.begin(), input.end(), [&valid] (auto i) {
        // Parse min and max
        int min = stoi_impl(i);
        i = strchr(i, '-')+1;
        int max = stoi_impl(i);

        // Seek to input char
        i = strchr(i, ' ')+1;

        char c = *i; // this is the spec
        int instances = 0;
        // Count instances of c (This will count ':' and ' ' but they will always fail so it doesn't matter)
        do {
            if (*i == c) {
                instances++;
            }
        } while (*++i);

        // Check that the total is within the spec
        if (min <= instances && instances <= max) {
            valid++;
        }
    });

    return valid;
}

// Use part 2 rules
template <typename T>
constexpr int day_02_2(T input){
    int valid = 0; // Count valid passwords

    // A spec for password rules is NN-NN X: xxxxxxxxxx....xxxxxxxxxx
    // NN is 1-2 digit int, min for total X's in input, second NN is max
    std::for_each(input.begin(), input.end(), [&valid] (auto i) {
        // Parse min and max
        int min = stoi_impl(i);
        i = strchr(i, '-')+1;
        int max = stoi_impl(i);

        // Seek to input char
        i = strchr(i, ' ')+1;

        char c = *i; // this is the spec
        int instances = 0;
        i++; // Seek to ':'
        i++; // Seek to ' '

        if (c == i[min]) instances++;
        if (c == i[max]) instances++;

        // Instances could be 0, 1, or 2. only valid if 1
        valid += instances % 2;

    });

    return valid;
}


#endif //AOC_2020_DAY_02_HPP
