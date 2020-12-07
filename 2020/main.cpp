//
// Created by Derek Witt on 12/2/20.
//

#include "day_01.hpp"
#include "day_01_input.hpp"
#include "day_02.hpp"
#include "day_02_input.hpp"
#include "day_03.hpp"
#include "day_03_input.hpp"
#include "day_04.h"
#include "day_04_input.h"
#include "day_05.h"
#include "day_05_input.h"
#include "day_06.h"
#include "day_06_input.h"
#include "day_07.h"
#include "day_07_input.h"
#include <fmt/format.h>

int main() {
    puts(fmt::format("Day1_1: {}", day_01_1(day_01_data)).c_str());
    puts(fmt::format("Day1_2: {}", day_01_2(day_01_data)).c_str());

    puts(fmt::format("Day2_1: {}", day_02_1(day_02_data)).c_str());
    puts(fmt::format("Day2_2: {}", day_02_2(day_02_data)).c_str());

    puts(fmt::format("Day3_1: {}", day_03_1(day_03_data)).c_str());
    puts(fmt::format("Day3_2: {}", day_03_2(day_03_data)).c_str());

    puts(fmt::format("Day4_1: {}", day_04_1(day_04_data)).c_str());
    puts(fmt::format("Day4_2: {}", day_04_2(day_04_data)).c_str());

    puts(fmt::format("Day5_1: {}", day_05_1(day_05_data)).c_str());
    puts(fmt::format("Day5_2: {}", day_05_2(day_05_data)).c_str());

    puts(fmt::format("Day6_1: {}", day_06_1(day_06_data)).c_str());
    puts(fmt::format("Day6_2: {}", day_06_2(day_06_data)).c_str());

    puts(fmt::format("Day7_1: {}", day_07_1(day_07_data)).c_str());
    puts(fmt::format("Day7_1: {}", day_07_2(day_07_data)).c_str());

}