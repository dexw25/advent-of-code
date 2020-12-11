//
// Created by Derek Witt on 12/10/20.
//

#include "day_10.h"
#include <array>

using namespace std;

int main() {
  constexpr array in{
      28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38,
      39, 11, 1,  32, 25, 35, 8,  17, 7,  9,  4,  2,  34, 10, 3,
  };
  assert(day_10_1(in) == 220);
  assert(day_10_2(in) == 19208);
}