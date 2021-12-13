#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        // Parse input lines as base 2 numbers
        let input = include_str!("test.txt")
            .lines()
            .map(|l| u64::from_str_radix(l, 2).unwrap());

        assert_eq!(crate::calculate_power_v1(input), 198);
    }
    #[test]
    fn part2() {
        // Parse input lines as base 2 numbers
        let input = include_str!("test.txt")
            .lines()
            .map(|l| u64::from_str_radix(l, 2).unwrap());

        assert_eq!(crate::calculate_power_v2(input), 230);
    }

    #[test]
    fn part2_real() {
        // Parse input lines as base 2 numbers
        let input = include_str!("input.txt")
            .lines()
            .map(|l| u64::from_str_radix(l, 2).unwrap());

        assert_eq!(crate::calculate_power_v2(input), 5410338);
    }
}

/// Calculate and return rates (gamma, epsilon). Could be a parameter here to provide a hint that only the lower N bits need to be considered
fn calculate_rates<T: Iterator<Item = u64>>(nums: T) -> (u64, u64) {
    let mut bit_counts: [u32; 64] = [0; 64];
    let mut length: usize = 0;
    let mut highest_bit_set: usize = 0;

    // Count how many bits are set in each of the 64 positions
    for n in nums {
        length += 1;
        let bits_set: usize = 64_usize - n.leading_zeros() as usize;

        if bits_set > highest_bit_set {
            highest_bit_set = bits_set;
        }

        for bit in 0..bits_set as usize {
            if n & (1 << bit) != 0 {
                bit_counts[bit] += 1;
            }
        }
    }

    let gamma_rate: u64 = bit_counts
        .iter()
        .enumerate()
        .map(|(i, val)| {
            if (length % 2 == 0) && *val == (length / 2).try_into().unwrap() {
                1 << i // edge case, if there is an equal distribution, set the current bit for gamma, clear for epsilon
            } else if *val > (length / 2).try_into().unwrap() {
                1 << i
            } else {
                0
            }
        })
        .sum();

    // Invert the part of the gamma rate that was derived from our data, and calculate the epsilon rate
    let mask = 0xFFFF_FFFF_FFFF_FFFF_u64 >> (64 - highest_bit_set);
    let epsilon_rate = (!gamma_rate) & mask;

    (gamma_rate, epsilon_rate)
}

/// Search for an entry in data that matches the given rate
fn search_with_rate(data: &[u64], popularity: bool) -> Result<u64, ()> {
    let mut test_bit: u64 = 1 << 12; // Start with left most bit and move right

    let mut matches = data.to_vec();
    while test_bit > 0 {
        // Find new search word from the current list, reuse old code to calculate most popular and unpopular bits in one step
        let (pop_rate, not_rate) = calculate_rates(matches.iter().copied());
        let rate = if popularity { pop_rate } else { not_rate };

        // Filter the data by the new search word and the specified search bit
        matches = matches
            .iter()
            .filter_map(|d| {
                // Check if the current bit agrees with the most popular value and filter
                if (*d & test_bit) == (rate & test_bit) {
                    Some(*d)
                } else {
                    None
                }
            })
            .collect();

        // End condition and also test mask adjustment
        if matches.len() == 1 {
            return Ok(matches[0]);
        } else {
            test_bit >>= 1; // search for next bit in next loop
        }

        // Panic if ran out of search bits
        if matches.len() == 0 {
            panic!();
        }
    }

    // If we get here, then there was no proper end to the search and this is also bad
    Err(())
}

fn calculate_power_v1<T: Iterator<Item = u64>>(nums: T) -> u64 {
    let (gamma_rate, epsilon_rate) = calculate_rates(nums);

    gamma_rate * epsilon_rate
}

fn calculate_power_v2<T: Iterator<Item = u64>>(nums: T) -> u64 {
    // We need to do multiple searches in the input, start by collecting
    let nums: Vec<u64> = nums.collect();

    let o2 = search_with_rate(&nums, true).unwrap();
    let co2 = search_with_rate(&nums, false).unwrap();

    o2 * co2
}

fn main() {
    // Parse input lines as base 2 numbers
    let input: Vec<u64> = include_str!("input.txt")
        .lines()
        .map(|l| u64::from_str_radix(l, 2).unwrap())
        .collect();

    println!("part1: {}", calculate_power_v1(input.iter().copied()));
    println!("part2: {}", calculate_power_v2(input.iter().copied()));
}
