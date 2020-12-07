//
// Created by Derek Witt on 12/7/20.
//
#include "day_07.h"

#include <vector>
#include <string_view>
#include <unordered_map>
#include <unordered_set>
#include <tuple>
#include <iostream>
#include <charconv>

using namespace std;

// Parse rule and return tuple suitable for the map in day_07_1
auto parse_rule(const string_view &in) {
    const string_view req = " bags contain ";
    auto begin = in.find(req); // get end of initial color

    auto key = in.substr(0, begin);
    vector<tuple<const string_view, int>> contents;

    // Set up for parsing the rest of the row
    begin += req.length(); // Advance to first rule
    size_t end = 0;

    // Special case: check for empty set
    if (in.find("no other bags", begin) == string_view::npos) {
        // For arbitrarily many rules in the line
        while (end != string_view::npos) {
            int n = 0;
            end = in.find(" bag", begin); // Split on bag, do not include bag
            auto spec = in.substr(begin, end - begin);

            // Extract number, do not accept errors
            auto[ptr, ec] = from_chars(spec.data(), spec.data() + spec.length(), n);
            if (ec != errc()) {
                throw;
            }

            // Extract color
            begin = in.find_first_not_of("0123456789 ", begin); // Seek to first letter
            spec = in.substr(begin, end - begin);

            // Save color and number to rules list
            contents.push_back({spec, n});

            // Find end of current rule and continue (comma separated list, period terminates but I just look to the end of the string)
            end = in.find(',', end);
            begin = end + 2; // Skip comma and space
        }
    }
    return make_tuple(key, contents);
}

auto parse_rules (const string_view &in) {
    // Keys are const reference into &in, values are lists of colors and the number of required bags, may be zero length
    unordered_map<string_view, vector<tuple<const string_view, int>>> rules;

    // Parse input string
    size_t offset = 0;
    size_t end = 0;

    // For each line in input (also for each rule)
    while (end != string_view::npos) {
        end = in.find('\n', offset);

        auto[top, contents] = parse_rule(in.substr(offset, end - offset));

        rules.insert({top, contents});

        offset = end + 1; // Star after delimiter
    }
    return rules;
}

// Find all nodes that contain the search key, directly or indirectly
void count_leaves(unordered_map<string_view, vector<tuple<const string_view, int>>> &rules, unordered_set<string_view> &roots,
const string_view &search) {
    // Traverse the rule tree(ish) for nodes that have leaves of "search"
    for (auto[key, rule]: rules) {
        // For every specifier in each rule row
        for (auto[color, n]: rule) {
            if (color == search) {
                roots.insert(key); // free if we found a leaf (or node that holds callers leaf)
                // Search for other nodes that point to this one, recursively
                count_leaves(rules, roots, key); // Does not recurse if search not found as leaf
            }
        }
    }
}

// Sum all recursive node values under the search term, could multiset if we want bag counts by color
int count_all_leaves(unordered_map<string_view, vector<tuple<const string_view, int>>> &rules,
                     const string_view &search, int mult=1){
    int acc = 0;

    // For every specifier in given rule (empty rule lists will cause 0 iterations here, that is recursion end condition
    for (auto[color, n]: rules[search]) {
        acc += n * mult;
        acc += count_all_leaves(rules, color, n * mult);
    }

    return acc;
}

// Read rules in and determine which bags can end in "shiny gold"
// rules are of the form '{color} bags contain N {color_a} bag[s] [, N {color_a} bag[s]] [[...]].'
int day_07_1(const string_view &in) {
    auto rules = parse_rules(in);

    // Set to hold all unique roots
    unordered_set<string_view> roots;

    count_leaves(rules, roots, "shiny gold");
    return roots.size(); // number of rules parsed as test
}

// Do the opposite, given the top level root, count the total bags that must be contained in the shiny gold one
int day_07_2(const string_view &in) {
    auto rules = parse_rules(in);

    int res = count_all_leaves(rules, "shiny gold");
    return res; // number of rules parsed as test
}