//
// Created by Derek Witt on 12/5/20.
//

#include <string_view>
#include "day_05.h"
#include "day_05_input.h"

using namespace std;

constexpr array<const string_view, 3> in = {"BFFFBBFRRR",
                                      "FFFBBBFRRR",
                                      "BBFFBBFRLL"};

int main() {
    static_assert(parse_pass(in[0]) == 567);
    static_assert(parse_pass(in[1]) == 119);
    static_assert(parse_pass(in[2]) == 820);

    static_assert(day_05_1(in) == 820);

    static_assert(day_05_1(day_05_data) == 955);
    static_assert(day_05_2(day_05_data) == 569);
}
