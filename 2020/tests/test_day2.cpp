//
// Created by Derek Witt on 12/2/20.
//

#include "day_02.hpp"
#include "day_02_input.hpp"

constexpr std::array<const char *, 3> in = {{
                                                    "1-3 a: abcde",
                                                    "1-3 b: cdefg",
                                                    "2-9 c: ccccccccc"
                                            }};

void day2_1_test() {
    static_assert(day_02_1(in) == 2);
}

void day2_2_test() {
    static_assert(day_02_2(in) == 1);
}

void day2_1_actual() {
    static_assert(day_02_1(day_02_data) == 401);
}

void day2_2_actual() {
    static_assert(day_02_2(day_02_data) == 688);
}

int main() {
    day2_1_test();
    day2_2_test();
    day2_1_actual();
    day2_2_actual();
}