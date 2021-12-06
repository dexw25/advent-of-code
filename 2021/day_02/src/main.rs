use std::cmp::*;
use std::str::FromStr;

#[cfg(test)]
mod day2_tests {
    #[test]
    fn direction_parse() {
        let dir: crate::Direction = "up".parse().unwrap();
        assert_eq!(dir, crate::Direction::Up);
        let dir: crate::Direction = "down".parse().unwrap();
        assert_eq!(dir, crate::Direction::Down);
        let dir: crate::Direction = "forward".parse().unwrap();
        assert_eq!(dir, crate::Direction::Forward);
    }

    #[test]
    fn move_parse() {
        let submove: crate::SubmarineMove = "forward 5".parse().unwrap();

        assert_eq!(
            submove,
            crate::SubmarineMove {
                direction: crate::Direction::Forward,
                distance: 5,
            }
        );
    }

    #[test]
    fn move_test() {
        let mut sub = crate::Submarine::new();
        sub.apply_move(&"forward 5".parse::<crate::SubmarineMove>().unwrap());

        assert_eq!(
            sub,
            crate::Submarine {
                depth: 0,
                horizontal_position: 5,
                aim: 0,
            }
        );
    }

    #[test]
    fn part1() {
        let input = include_str!("test.txt");
        let mut sub = crate::Submarine::new();

        let _num_moves = input
            .lines()
            .map(|l| l.parse().unwrap())
            .map(|m: crate::SubmarineMove| sub.apply_move(&m))
            .count();

        assert_eq!(sub.get_product(), 150);
    }

    #[test]
    fn part2() {
        let input = include_str!("test.txt");
        let mut sub = crate::Submarine::new();

        let _num_moves = input
            .lines()
            .map(|l| l.parse().unwrap())
            .map(|m: crate::SubmarineMove| sub.apply_move_v2(&m))
            .count();

        assert_eq!(sub.get_product(), 900);
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "forward" => Ok(Self::Forward),
            _ => Err(()),
        }
    }
}

impl FromStr for SubmarineMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        let direction = if let Some(dir) = tokens.next() {
            dir.parse()?
        } else {
            return Err(());
        };

        let distance = if let Some(num) = tokens.next() {
            num.parse().unwrap()
        } else {
            return Err(());
        };

        Ok(Self {
            direction,
            distance,
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
struct SubmarineMove {
    direction: Direction,
    distance: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct Submarine {
    depth: usize,
    horizontal_position: usize,
    aim: usize,
}

impl Submarine {
    fn new() -> Self {
        Self {
            depth: 0,
            horizontal_position: 0,
            aim: 0,
        }
    }

    fn apply_move(&mut self, directive: &SubmarineMove) {
        use Direction::*;
        match directive.direction {
            Forward => self.horizontal_position += directive.distance,
            Up => self.depth -= directive.distance,
            Down => self.depth += directive.distance,
        }
    }

    /// Apply moves considering aim
    fn apply_move_v2(&mut self, directive: &SubmarineMove) {
        use Direction::*;
        match directive.direction {
            Forward => {
                self.horizontal_position += directive.distance;
                self.depth += directive.distance * self.aim;
            }
            Up => self.aim -= directive.distance,
            Down => self.aim += directive.distance,
        }
    }

    // Part 1 solution (along with all of the machinery that precedes)
    fn get_product(&self) -> usize {
        self.depth * self.horizontal_position
    }
}

fn part1(moves: &Vec<SubmarineMove>) -> usize {
    let mut sub = Submarine::new();

    let _num_moves = moves.iter().map(|m| sub.apply_move(m)).count();

    sub.get_product()
}
fn part2(moves: &Vec<SubmarineMove>) -> usize {
    let mut sub = Submarine::new();

    let _num_moves = moves.iter().map(|m| sub.apply_move_v2(m)).count();

    sub.get_product()
}

fn main() {
    let input = include_str!("input.txt");

    let moves: Vec<SubmarineMove> = input.lines().map(|m| m.parse().unwrap()).collect();

    println!("Part1: {}", part1(&moves));
    println!("Part1: {}", part2(&moves));
}
