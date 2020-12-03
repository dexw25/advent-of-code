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

void day2_1_test()
{
    constexpr auto out = day_02_1(in);
    static_assert( out == 2);
}

void day2_2_test()
{
    constexpr int out = day_02_2(in);
    static_assert( out == 1);
}

void day2_1_actual(){
    constexpr int out = day_02_1(day_02_data);
    static_assert (out == 401);
}

void day2_2_actual(){
    constexpr int out = day_02_2(day_02_data);
    static_assert (out == 688);
}

int main() {
    day2_1_test();
    day2_2_test();
    day2_1_actual();
    day2_2_actual();
}