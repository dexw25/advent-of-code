//
// Created by Derek Witt on 12/2/20.
//

#include "day_01.hpp"
#include "day_01_input.hpp"
#include "day_02.hpp"
#include "day_02_input.hpp"
#include "day_03.hpp"
#include "day_03_input.hpp"
#include <fmt/format.h>

int main() {
    constexpr auto res1_1 = day_01_1(day_01_data);
    puts(fmt::format("Day1_1: {}", res1_1).c_str());

    constexpr auto res1_2 = day_01_2(day_01_data);
    puts(fmt::format("Day1_2: {}", res1_2).c_str());

    constexpr auto res2_1 = day_02_1(day_02_data);
    puts(fmt::format("Day2_1: {}", res2_1).c_str());

    constexpr auto res2_2 = day_02_2(day_02_data);
    puts(fmt::format("Day2_2: {}", res2_2).c_str());

    constexpr auto res3_1 = day_03_1(day_03_data);
    puts(fmt::format("Day3_1: {}", res3_1).c_str());

    constexpr auto res3_2 = day_03_2(day_03_data);
    puts(fmt::format("Day3_2: {}", res3_2).c_str());
}