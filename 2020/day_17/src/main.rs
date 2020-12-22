mod data;
use crate::data::DATA;
use itertools::Itertools;
use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

struct Pocket {
    // Set of all cubes that are on in the current state
    cubes: HashSet<(i32, i32, i32, i32)>,
}

impl FromStr for Pocket {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    cubes.insert((x as i32, y as i32, 0, 0));
                }
            }
        }
        Ok(Pocket { cubes })
    }
}

impl Pocket {
    fn count_adjacent(&self, point: &(i32, i32, i32)) -> i32 {
        let (x, y, z) = point;
        (x - 1..=x + 1)
            .cartesian_product((y - 1..=y + 1).cartesian_product(z - 1..=z + 1))
            .filter_map(|(x, (y, z))| {
                if &(x, y, z) != point && self.cubes.get(&(x, y, z, 0)).is_some() {
                    Some(1)
                } else {
                    None
                }
            })
            .sum()
    }
    fn count_adjacent_4d(&self, point: &(i32, i32, i32, i32)) -> i32 {
        let (x, y, z, w) = point;
        (x - 1..=x + 1)
            .cartesian_product(
                (y - 1..=y + 1).cartesian_product((z - 1..=z + 1).cartesian_product(w - 1..=w + 1)),
            )
            .filter_map(|(x, (y, (z, w)))| {
                if &(x, y, z, w) != point && self.cubes.get(&(x, y, z, w)).is_some() {
                    Some(1)
                } else {
                    None
                }
            })
            .sum()
    }

    fn step(&mut self, fourd: bool) {
        let mut next_state: HashSet<(i32, i32, i32, i32)> = HashSet::new();

        // Iterate over all points that are active, and all their neighbors, special case for 4d
        if fourd {
            for (x, (y, (z, w))) in self.cubes.iter().flat_map(|(x, y, z, w)| {
                (x - 1..=x + 1)
                    .cartesian_product(
                        (y - 1..=y + 1)
                            .cartesian_product((z - 1..=z + 1).cartesian_product(w - 1..=w + 1)),
                    )
                    .collect::<HashSet<_>>() // hash set to eliminate duplicates
            }) {
                // Check if cube is active or inactive, and process state changes accordingly
                if self.cubes.get(&(x, y, z, w)).is_some() {
                    if (2..=3).contains(&self.count_adjacent_4d(&(x, y, z, w))) {
                        next_state.insert((x, y, z, w));
                    }
                // No else for dead cube case, absence signals death
                } else if self.count_adjacent_4d(&(x, y, z, w)) == 3 {
                    next_state.insert((x, y, z, w)); // activate cube
                }
            }
        } else {
            for (x, (y, z)) in self.cubes.iter().flat_map(|(x, y, z, _w)| {
                (x - 1..=x + 1)
                    .cartesian_product((y - 1..=y + 1).cartesian_product(z - 1..=z + 1))
                    .collect::<HashSet<_>>() // hash set to eliminate duplicates
            }) {
                // Check if cube is active or inactive, and process state changes accordingly
                if self.cubes.get(&(x, y, z, 0)).is_some() {
                    if (2..=3).contains(&self.count_adjacent(&(x, y, z))) {
                        next_state.insert((x, y, z, 0));
                    }
                // No else for dead cube case, absence signals death
                } else if self.count_adjacent(&(x, y, z)) == 3 {
                    next_state.insert((x, y, z, 0)); // activate cube
                }
            }
        }

        self.cubes = next_state;
    }

    fn run(&mut self, steps: u32, fourd: bool) {
        for _ in 0..steps {
            self.step(fourd);
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut p = input.parse::<Pocket>().unwrap();

    p.run(6, false); // Run 6 cycles

    p.cubes.len()
}

fn part_2(input: &str) -> usize {
    let mut p = input.parse::<Pocket>().unwrap();

    p.run(6, true); // Run 6 cycles in 4D mode

    p.cubes.len()
}

fn main() {
    println!("p1: {}", part_1(DATA));
    println!("p2: {}", part_2(DATA));
}

#[test]
fn test_p1() {
    let input = ".#.
..#
###";

    assert_eq!(part_1(input), 112);
}
#[test]
fn test_p2() {
    let input = ".#.
..#
###";

    assert_eq!(part_2(input), 848);
}
