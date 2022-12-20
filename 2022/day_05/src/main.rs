mod supply_stacks {
    use color_eyre::{eyre::eyre, Report, Result};
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::{fmt::Display, str::FromStr};

    #[derive(Debug, Clone, Copy)]
    pub struct Container {
        identifier: char,
    }

    impl FromStr for Container {
        type Err = Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // Check input first
            if s.len() != 3 {
                Err(eyre!("Input is the wrong length"))
            } else if !(s.starts_with('[') && s.ends_with(']')) {
                Err(eyre!("Input should be exactly of the form [_]"))
            } else {
                // All checks pass, simply extract the middle character
                Ok(Self {
                    identifier: s.chars().nth(1).ok_or(eyre!("Not enough input!"))?,
                })
            }
        }
    }

    impl Display for Container {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}]", self.identifier)
        }
    }

    #[derive(Clone)]
    pub struct Ship {
        /// Represent stacks as a list of lists
        stacks: Vec<Vec<Container>>,
    }

    impl Ship {
        pub fn apply_move(&mut self, todo: &Move) -> Result<()> {
            for _i in 0..todo.num_crates {
                let temp = self.stacks[todo.source - 1]
                    .pop()
                    .ok_or(eyre!("stack empty!"))?;
                self.stacks[todo.dest - 1].push(temp);
            }
            Ok(())
        }
        pub fn apply_move_v2(&mut self, todo: &Move) -> Result<()> {
            let mut temp = Vec::with_capacity(todo.num_crates);

            // Pop crates into temporary vector
            for _i in 0..todo.num_crates {
                temp.push(
                    self.stacks[todo.source - 1]
                        .pop()
                        .ok_or(eyre!("Move emptied the stack?"))?,
                );
            }
            // Push onto destination in appropriate order
            self.stacks[todo.dest - 1].extend(temp.iter().rev());

            Ok(())
        }

        pub fn read_top(&self) {
            for stack in &self.stacks {
                print!("{}", stack[stack.len() - 1]);
            }
            println!();
        }
    }

    impl Display for Ship {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(height) = self.stacks.iter().map(Vec::len).max() {
                // Top down iteration
                for i in 0..height {
                    for stack in &self.stacks {
                        if let Some(container) = stack.get(height - 1 - i) {
                            write!(f, "{container} ")?;
                        } else {
                            write!(f, "    ")?;
                        }
                    }
                    writeln!(f)?;
                }
                // Ship contents printed, write out label row
                for i in 1..=self.stacks.len() {
                    if i < self.stacks.len() {
                        write!(f, " {i}  ")?;
                    } else {
                        // Do not write out an extra garbage space on the last row
                        // This Display impl matches the
                        write!(f, " {i} ")?;
                    }
                }
                Ok(())
            } else {
                write!(f, "<empty ship>")
            }
        }
    }

    impl FromStr for Ship {
        type Err = Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // Find the bottom row, which has the stack numbers, and extract the highest index
            // This will size the list of lists
            let num_stacks = s
                .lines()
                .last()
                .ok_or(eyre!("No ship?"))?
                .split_whitespace()
                .last()
                .ok_or(eyre!("No stacks?"))?
                .parse::<usize>()?;

            // Initialize a vec or each stack, this could be represented in memory slightly more tightly but this is good enough
            let mut stacks = Vec::with_capacity(num_stacks);
            for _i in 0..num_stacks {
                stacks.push(Vec::new());
            }

            // Go from the bottom up, skip the first row which labels each stack
            for layer in s.lines().rev().skip(1) {
                // Iterate through stacks and index the string to dig out containers
                for (i, stack) in stacks.iter_mut().enumerate() {
                    // A plain index here could work, but I don't have a guarantee the input will always be whitespace padded to the full width
                    // So by extracting the substrings this way, garbage at the end will be thrown out (and missing whitespace will not be missed)
                    if let Some(slot) = layer.get(i * 4..i * 4 + 3) {
                        match slot {
                            "   " => continue,
                            container => stack.push(container.parse()?),
                        }
                    }
                }
            }

            Ok(Self { stacks })
        }
    }

    #[derive(Copy, Clone)]
    pub struct Move {
        num_crates: usize,
        source: usize,
        dest: usize,
    }

    // this parser is very dumb and honestly borderline unsound. Fuck handling Nom's errors
    impl FromStr for Move {
        type Err = color_eyre::Report;

        // The form of this is "move x from y to z" where x y and z are all positive numbers
        // Could be more efficient using nom but fuckit nom is confusing
        #[allow(clippy::unwrap_used)]
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // Compile our expression first (well, really this only happens once)
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
            };

            let bits = RE.captures(s).ok_or(eyre!("No match found in input"))?;

            Ok(Self {
                num_crates: bits
                    .get(1)
                    .ok_or(eyre!("malformed input"))?
                    .as_str()
                    .parse()?,
                source: bits
                    .get(2)
                    .ok_or(eyre!("malformed input"))?
                    .as_str()
                    .parse()?,
                dest: bits
                    .get(3)
                    .ok_or(eyre!("malformed input"))?
                    .as_str()
                    .parse()?,
            })
        }
    }

    impl Display for Move {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "move {} from {} to {}",
                self.num_crates, self.source, self.dest
            )
        }
    }
}

use color_eyre::{eyre::eyre, Result};
use supply_stacks::{Move, Ship};
fn main() -> Result<()> {
    let mut input = include_str!("input.txt").split("\n\n");

    let mut ship: Ship = input.next().ok_or(eyre!("no ship?"))?.parse()?;
    let mut ship2 = ship.clone();
    println!("Starting ship:");
    println!();
    println!("{ship}");

    let moves = input
        .next()
        .ok_or(eyre!("no moves?"))?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Move>, _>>()?;

    for i in &moves {
        ship.apply_move(i)?;
    }

    ship.read_top();
    for i in &moves {
        ship2.apply_move_v2(i)?;
    }

    ship2.read_top();
    Ok(())
}
