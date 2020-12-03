//
// Created by Derek Witt on 12/2/20.
//

#include "day_01.hpp"
#include "day_01_input.hpp"
#include <array>

constexpr std::array<int, 6> in = {1721,
                         979,
                         366,
                         299,
                         675,
                         1456,};

void day1_1_test()
{
    // Test that constexpr evaluation works with constexpr inputs
    static_assert( day_01_1(in) == 514579);
}

void day1_1_actual() {
    static_assert(day_01_1(day_01_data) == 138379);
}

void day1_2_test()
{
    static_assert( day_01_2(in) == 241861950);
}

void day1_2_actual() {
    static_assert(day_01_2(day_01_data) == 85491920);
}

int main() {
    day1_1_test();
    day1_1_actual();
    day1_2_test();
    day1_2_actual();
}