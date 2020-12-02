//
// Created by Derek Witt on 12/2/20.
//

#include "day_01.hpp"
#include "day_01_input.hpp"
#include <fmt/format.h>

int main() {
    constexpr auto res1_1 = day_01_1(day_01_data);
    puts(fmt::format("Day1_1: {}", res1_1).c_str());

    constexpr auto res1_2 = day_01_2(day_01_data);
    puts(fmt::format("Day1_2: {}", res1_2).c_str());

}