//
// Created by Derek Witt on 12/10/20.
//

#ifndef AOC_2020_DAY_10_H
#define AOC_2020_DAY_10_H
#include <algorithm>
#include <array>

using namespace std;

// Pass array by copy to allow modification
// This would be constexpr except for appleclang being stuck at an older version of LLVM (10 vs 11)
template <typename Array>
int day_10_1(const Array &in) {
  int ones = 0, threes = 0;

  auto tmp = in;
  sort(begin(tmp), end(tmp));

  // Go through and sum the 1-steps and the 3-steps
  for (auto it = tmp.begin() + 1; it < tmp.end(); it++) {
    if ((*it - *(it - 1)) == 1) {
      ones++;
    } else if ((*it - *(it - 1)) == 3) {
      threes++;
    } else if ((*it - *(it - 1)) != 2) {
      throw; // Handle special case of bad input
    }
  }
  threes++; // Count the final step to the phone

  // Handle first element to 0
  if (tmp[0] == 1)
    ones++;
  else if (tmp[0] == 3)
    threes++;

  return ones * threes;
}

// Sum of previous 3 numbers in a sequence
// Dynamic programming/memoization/caching could speed this up I guess
constexpr int tribonacci(int i) {
  if (i <= 1)
    return 0;
  else if (i == 2)
    return 1;

  // 3 and above, do the math
  return tribonacci(i - 1) + tribonacci(i - 2) + tribonacci(i - 3);
};

// count all of the possible combinations of chargers that add to the highest
// number
template <size_t N> constexpr long day_10_2(const array<int, N> &in) {
  array<int, N> tmp = in;
  int ones = 0;
  long combinations = 1;
  sort(begin(tmp), end(tmp));

  // Figure in initial element
  if (tmp[0] == 1)
    ones++; // else ones = 0

  for (auto it = tmp.begin() + 1; it < tmp.end(); it++) {
    int diff =
        *it -
        *(it - 1); // difference between this and the last one is what matters

    // diff will be 1 or 3, runs of 1's are what determine the number of new
    // branches this introduces
    if (diff == 2)
      throw; // This is an assumption of input data, make sure it holds

    // Count runs of 1's and figure out how many branches there are
    if (diff == 1) {
      ones++;
    } else if (diff == 3) {
      if (ones) {
        // Worked it out on paper, this is a tribonacci sequence more or less
        combinations *= tribonacci(ones + 2);
      }
      ones = 0;
    }
  }

  // Figure in end
  if (ones)
    combinations *= tribonacci(ones + 2);

  return combinations;
}

// Count the total number of possible combinations that is valid

#endif // AOC_2020_DAY_10_H
