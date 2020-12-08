//
// Created by Derek Witt on 12/8/20.
//

#include "day_08.h"
#include "day_08_input.h"
#include <string_view>
using namespace std;

constexpr string_view in = "nop +0\n"
                           "acc +1\n"
                           "jmp +4\n"
                           "acc +3\n"
                           "jmp -3\n"
                           "acc -99\n"
                           "acc +1\n"
                           "jmp -4\n"
                           "acc +6";

int main() {
    static_assert(day_08_1(in) == 5);
    static_assert(day_08_2(in) == 8);

    static_assert(day_08_1(day_08_data) == 1753);
    static_assert(day_08_2(day_08_data) == 733);
}