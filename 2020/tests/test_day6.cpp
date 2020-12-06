//
// Created by Derek Witt on 12/5/20.
//

#include "day_06.h"
#include <string_view>
using namespace std;

constexpr string_view in = "abc\n"
                           "\n"
                           "a\n"
                           "b\n"
                           "c\n"
                           "\n"
                           "ab\n"
                           "ac\n"
                           "\n"
                           "a\n"
                           "a\n"
                           "a\n"
                           "a\n"
                           "\n"
                           "b";

int main() {
    static_assert(yes_in_grp("abc") == 3);
    static_assert(yes_in_grp("a\nb\nc") == 3);
    static_assert(yes_in_grp("a\na\na") == 1);

    static_assert(day_06_1(in) == 11);
    static_assert(day_06_2(in) == 6);
}