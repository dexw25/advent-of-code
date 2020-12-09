//
// Created by Derek Witt on 12/9/20.
//
#include "day_09.h"
#include "day_09_input.h"
#include <array>

using namespace std;
constexpr array<int, 20> in = {35,  20,  15,  25,  47,  40,  62,
                               55,  65,  95,  102, 117, 150, 182,
                               127, 219, 299, 277, 309, 576};

int main() {
  static_assert(day_09_1(in, 5) == 127);
  static_assert(day_09_1(day_09_data, 25) == 1930745883);

  static_assert(day_09_2(in, 5) == 62);
  static_assert(day_09_2(day_09_data, 25) == 268878261);
}