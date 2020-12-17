use std::convert::TryInto;

mod data;

struct Ferry {
    state: Vec<char>,
    next: Vec<char>,
    width: i32,
}

// Methods to parse the input, to iterate the state of the game of life, to access the internal vector with default values
impl Ferry {
    fn new(input: &str) -> Ferry {
        let mut w = 0;
        // Count row length
        for c in input.chars() {
            w += 1;
            if c == '\n' { break }
        }

        Ferry {
            state: input.chars().collect(),
            next: input.chars().collect(),
            width: w, // Make it so adding this selects the next row, and subtracting the previous row
        }
    }
    // Address the grid with bounds checking (do not mask '\n' here)
    fn get(&self, i: i32) -> char {
        if i < 0 {
            'L' // Undefined defaults to empty seat, so that it adds no full seats but also terminates rays
        } else if i < self.state.len() as i32 {
            self.state[i as usize]
        } else {
            'L'
        }
    }

    // Sum all instances of a given character in an array
    fn count_all(&self, c: char) -> u32 {
        let mut acc = 0;
        for i in self.state.iter() {
            if *i == c {
                acc += 1;
            }
        }
        acc
    }

    // Count adjacent instances of a given character, using masking for '/n'
    fn count_adjacent(&self, i: i32, c: char) -> u32 {
        let mut acc = 0;
        let check_ids: [i32; 8] = [
            i - 1 as i32,
            i + 1 as i32,
            i - self.width - 1,
            i - self.width,
            i - self.width + 1,
            i + self.width - 1,
            i + self.width,
            i + self.width + 1,
        ];

        // Sum the matches
        for j in check_ids.iter() {
            acc += {
                if self.get(*j) == c {
                    1
                } else {
                    0
                }
            }
        }

        acc
    }

    // Count rays that terminate in empty or full seats
    fn count_by_ray(&self, i: i32) -> u32 {
        let mut acc = 0;
        // Array of x,y pointers to guide the counting loop below
        let check_rays: [(i32, i32); 8] = [
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, 1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        // Sum the matches
        for (dx, dy) in check_rays.iter() {
            let mut j = i;
            acc += loop {
                j += dx; // step column
                j += dy * self.width; // step row

                match self.get(j) {
                    'L' | '\n' => { break 0; }
                    '#' => { break 1; }
                    '.' => { continue; }
                    _ => { panic!("Bad input") }
                }
            }
        }

        acc
    }

    // step the simulation and return the number of cells changed by ray casting
    fn step_ray(&mut self) -> u32 {
        let mut acc: u32 = 0;
        for (i, c) in self.state.iter().enumerate() {
            acc += match *c {
                '.' | '\n' => {
                    self.next[i] = *c;
                    0
                }
                'L' => {
                    if self.count_by_ray(i.try_into().unwrap()) == 0 {
                        self.next[i] = '#';
                        1
                    } else {
                        self.next[i] = *c;
                        0
                    }
                }
                '#' => {
                    if self.count_by_ray(i.try_into().unwrap()) >= 5 {
                        self.next[i] = 'L';
                        1
                    } else {
                        self.next[i] = *c;
                        0
                    }
                }
                _ => { panic!("Bad Input!") }
            }
        }
        for (i, c) in self.next.iter().enumerate() {
            self.state[i] = *c;
        }

        acc
    }

    // step the simulation and return the number of cells changed
    fn step(&mut self) -> u32 {
        let mut acc: u32 = 0;
        for (i, c) in self.state.iter().enumerate() {
            acc += match *c {
                '.' | '\n' => {
                    self.next[i] = *c;
                    0
                }
                'L' => {
                    if self.count_adjacent(i.try_into().unwrap(), '#') == 0 {
                        self.next[i] = '#';
                        1
                    } else {
                        self.next[i] = *c;
                        0
                    }
                }
                '#' => {
                    if self.count_adjacent(i.try_into().unwrap(), '#') >= 4 {
                        self.next[i] = 'L';
                        1
                    } else {
                        self.next[i] = *c;
                        0
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
        for (i, c) in self.next.iter().enumerate() {
            self.state[i] = *c;
        }

        acc
    }
}

fn day_11_1(input: &str) -> u32 {
    let mut f = Ferry::new(input); // width is offset to next row from current row
    let mut steps = 0;
    while f.step() != 0 { steps += 1 }

    println!("{} Steps were simulated", steps);

    f.count_all('#')
}

fn day_11_2(input: &str) -> u32 {
    let mut f = Ferry::new(input); // width is offset to next row from current row
    let mut steps = 0;
    while f.step_ray() != 0 { steps += 1 }

    println!("{} Steps were simulated", steps);

    f.count_all('#')
}


fn main() {
    println!("Part 1: {}", day_11_1(data::DATA));
    println!("Part 2: {}", day_11_2(data::DATA));
}

#[test]
fn test1() {
    let input: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    assert_eq!(day_11_1(input), 37);
}

#[test]
fn test2() {
    let input: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    assert_eq!(day_11_2(input), 26);
}