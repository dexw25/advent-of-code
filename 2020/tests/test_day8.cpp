//
// Created by Derek Witt on 12/8/20.
//

#include "day_08.h"
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
    assert(day_08_1(in) == 5);
    assert(day_08_2(in) == 8);
}