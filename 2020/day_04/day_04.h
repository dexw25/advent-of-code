//
// Created by Derek Witt on 12/4/20.
//

#ifndef AOC_2020_DAY_04_H
#define AOC_2020_DAY_04_H

#include <string>
#include <array>
#include <span>
#include <unordered_set>
#include <charconv>
#include <cctype>
#include <iostream>

#include "fmt/format.h"

#include "day_02.hpp"

using namespace std;

enum Keys {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID
};

// Map keys to element types
constexpr array<const string_view, 8> keys = {
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        "cid",
};

constexpr array<const string_view, 7> colors{
        "amb",
        "blu",
        "brn",
        "gry",
        "grn",
        "hzl",
        "oth",
};

// passport element base class
struct PassportElement {
    string_view val_str;
    string_view key;


    [[nodiscard]] int constexpr parse_value(size_t offset = 0) const {
        int val;
        val = stoi_impl(this->val_str.data() + offset);

        return val;
    }

    [[nodiscard]] bool constexpr min_max(int min, int max) const {
        int val = this->parse_value();
        if (val >= min && val <= max) {
            return true;
        } else {
            return false;
        }
    }

    // This is ugly but inheritance and polymorphism was uglier
    constexpr bool validate() {
        int chars = 0;
        for (size_t i = 0; i < keys.size(); i++) {
            if (keys[i] == this->key) {
                switch (i) {
                    case BYR:
                        return this->min_max(1920, 2002);
                    case IYR:
                        return this->min_max(2010, 2020);
                    case EYR:
                        return this->min_max(2020, 2030);
                    case HGT:
                        if (this->val_str.substr(this->val_str.size() - 2, 2) == "cm") {
                            return this->min_max(150, 193);
                        } else if (this->val_str.substr(this->val_str.size() - 2, 2) == "in") {
                            return this->min_max(59, 76);
                        }
                        break;
                    case HCL:
                        // Count valid chars, should be exactly six after initial hash
                        if (this->val_str[0] == '#') {
                            for (auto j: this->val_str) {
                                if (is_hexdigit(j)) chars++;
                            }
                            if (chars == 6) {
                                return true;
                            } else {
                                break;
                            }
                        }
                        return false;
                    case ECL:
                        for (auto j: colors) {
                            if (j == this->val_str) {
                                return true;
                            }
                        }
                        break;
                    case PID:
                        for (auto j: this->val_str) {
                            if (is_digit(j)) chars++;
                        }
                        if (chars == 9) {
                            return true;
                        } else {
                            break;
                        }
                    case CID:
                        return true;
                    default:
                        break;
                }
            }
        }

        return false;
    };

// construct from key:value pair
    explicit constexpr PassportElement(const string_view &in) {
        size_t div = in.find(':');
        this->key = in.substr(0, div); // Key is part of string before ':'
        this->val_str = in.substr(div + 1, in.size() - div - 1); // captures : to char before whitespace (excluding ':')
    }

};

// Class to hold the contents of a passport and orchestrate validation
// Currently there is no handling of repeated fields since the input doesn't seem to have any
struct Passport {
    // for part 1
    size_t num_elements = 0;
    size_t valid_elements = 0;

    // More intensive validation for part 2
    bool valid;

    // Read a string and parse as passports, validate input data too
    constexpr explicit Passport(const string_view &in) {
        size_t offset = 0;
        size_t next_delim = in.find_first_of(" \n", offset);

        while (true) {
            PassportElement el(in.substr(offset, next_delim - offset));

            // Count all elements
            this->num_elements++;

            // Validate using derived functionality
            if (el.validate()) this->valid_elements++;

            // End condiiton, if this is the last element, break
            if (next_delim == in.size()) break;

            // advance offset and seek to end of next element
            offset = next_delim + 1;

            next_delim = in.find_first_of(" \n", offset);
            // Special handling for end of input, continue one more time when we hit the end
            if (next_delim == string_view::npos) {
                next_delim = in.size();
            }
        }

        // Pretend missing cid is present if missing
        if (in.find("cid:") == string_view::npos) {
            this->num_elements++;
            this->valid_elements++;
        }

        this->valid = (valid_elements == 8);
    }

};

// Parse the input and count total elements (no field validation)
constexpr int day_04_1(const string_view &in) {
    size_t offset = 0;
    int valid_passes = 0;
    // Read passports from input
    size_t end = 0;
    while (end != string_view::npos) {
        end = in.find("\n\n", offset);
        Passport p(in.substr(offset, end - offset));
        offset = end + 2; // advance past delimiter

        if (p.num_elements == 8) {
            valid_passes++;
        } else {
        }
    }

    return valid_passes;
}

// Parse the input and validate fields
constexpr int day_04_2(const string_view &in) {
    size_t offset = 0;
    int valid_passes = 0;
    // Read passports from input
    size_t end = 0;
    while (end != string_view::npos) {
        end = in.find("\n\n", offset);
        Passport p(in.substr(offset, end - offset));
        offset = end + 2; // advance past delimiter
        if (p.valid) {
            valid_passes++;
        }
    }

    return valid_passes;
}

#endif //AOC_2020_DAY_04_H
