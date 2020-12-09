//
// Created by Derek Witt on 12/9/20.
//

#ifndef AOC_2020_DAY_09_H
#define AOC_2020_DAY_09_H

#include <array>
#include <iostream>
#include <string_view>
#include <limits>

using namespace std;

// Check if there are two such numbers in the input stream that add to the
// passed number Input is the latest number and the sequence preceding, sequence
// length is N
template <typename T>
constexpr bool is_valid_seq(const T *begin, const T *end) {
  // use ptrs and ptr arithmetic
  T key = *end; // Get last number, no need to move back since loops are < not <=
  int i = 0;
  for (auto it = begin; it < end - 1; it++) {
    // Since we only care for combinations, start at the iterator after the
    // current one

    for (auto jt = it + 1; jt < end; jt++) {
      i++;
      if ((*it + *jt) == key) {
        // Any hit is a good hit
        return true;
      }
    }
  }
  return false;
}

template <typename T, size_t N>
constexpr T day_09_1(const array<T, N> &in, size_t pre_len) {
  for (size_t i = 0; i + pre_len < in.size(); i++) {
    // Get ptr to current int and candidate int
    if (!is_valid_seq(in.data() + i, in.data() + i + pre_len))
      return in[i + pre_len];
  }
  throw; // Exception if nothing found
}

template <typename T, size_t N>
T constexpr day_09_2(const array<T, N> &in, size_t pre_len) {
  T key = day_09_1(in, pre_len);

  // Search the input sequence for a contiguous set of 2+ number that sum to key
  // return min+max of this set
  auto tail = in.begin();
  auto head = tail + 1;
  T acc;

  // Caterpillar, advance head and tail ptrs until a sequence long enough is found
  while(head < in.end()){
    // Should never happen
    if (tail > head) throw;

    acc=0;
    // Sum all things, including both ends
    for_each(tail, head+1, [&acc] (auto i) {acc += i;});
    if (head - tail >=1 && acc == key) {
      return (*max_element(tail, head+1) + *min_element(tail, head+1));
    } else if (acc > key) {
      tail++;
    } else if (acc < key) {
      head++;
    }
  }

  throw;
}

#endif // AOC_2020_DAY_09_H
