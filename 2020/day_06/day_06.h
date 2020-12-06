//
// Created by Derek Witt on 12/5/20.
//

#ifndef AOC_2020_DAY_06_H
#define AOC_2020_DAY_06_H

#include <string_view>
using namespace std;

// Count number of any questions that had a yes
constexpr int yes_in_grp(const std::string_view &grp) {
    bool yes[26] = {false}; // Bit for if someone answered yes to a given question, one slot for each letter
    int ret = 0;

    // use c to index into grp, clean data guarantees no out of bounds access
    for(auto c: grp) {
        // Newlines may be in the data, ignore all chars not in question space
        if ('a' <= c && c <= 'z') {
            yes[c - 'a'] = true;
        }
    }

    // Count unique yes's
    for(auto y: yes){
        if (y) ret++;
    }

    return ret;
}

// Very similar to above, except count the number of questions that every group member answered yes to
constexpr int all_yes_in_grp(const std::string_view &grp) {
    int yes[26] = {0}; // Count of yes answers within the group
    int ret = 0;
    int people = 1; // at least one person

    // use c to index into grp, clean data guarantees no out of bounds access
    for(auto c: grp) {
        // Newlines may be in the data, ignore all chars not in question space
        if ('a' <= c && c <= 'z') {
            yes[c - 'a']++;
        } else if (c == '\n') { // each newline is a new person
            people++;
        }
    }

    // Count fields for which yes's == number of people
    for(auto y: yes){
        if (y == people) ret++;
    }

    return ret;
}

constexpr int day_06_1(const std::string_view &in) {
    int ret = 0;
    size_t offset = 0;
    size_t end = 0;

    while (end != string_view::npos) {
        end = in.find("\n\n", offset);

        ret += yes_in_grp(in.substr(offset, end - offset));

        offset = end + 2; // advance past delimiter
    }

    return ret;
}

constexpr int day_06_2(const std::string_view &in) {
    int ret = 0;
    size_t offset = 0;
    size_t end = 0;

    while (end != string_view::npos) {
        end = in.find("\n\n", offset);

        ret += all_yes_in_grp(in.substr(offset, end - offset));

        offset = end + 2; // advance past delimiter
    }

    return ret;
}

#endif //AOC_2020_DAY_06_H
