use std::str::FromStr;

use color_eyre::{eyre::eyre, Report, Result};

// rock beats scissor
// paper beats rock
// scissor beats paper

// input is of the form {move} {response}
// Use PartialOrd to hack the comparison
#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

// Score values for shapes
impl From<Move> for u32 {
    fn from(value: Move) -> Self {
        match value {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl FromStr for Outcome {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(eyre!("Unrecognized outcome code {s}")),
        }
    }
}

// Score values of each of these
impl From<Outcome> for u32 {
    fn from(value: Outcome) -> Self {
        match value {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

impl Move {
    pub const fn fight(self, other: Self) -> Outcome {
        use Move::{Paper, Rock, Scissors};
        use Outcome::{Draw, Loss, Win};

        match self {
            Rock => match other {
                Rock => Draw,
                Paper => Loss,
                Scissors => Win,
            },
            Paper => match other {
                Rock => Win,
                Paper => Draw,
                Scissors => Loss,
            },
            Scissors => match other {
                Rock => Loss,
                Paper => Win,
                Scissors => Draw,
            },
        }
    }
}

impl FromStr for Move {
    type Err = Report;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(eyre!("Invalid character {value}")),
        }
    }
}

fn main() -> Result<()> {
    let scores: Result<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|l| -> Result<u32> {
            let mut moves = l.split_whitespace().map(str::parse);

            let opening: Move = moves.next().ok_or(eyre!("No moves in line"))??;
            let response: Move = moves.next().ok_or(eyre!("Only one move in line"))??;

            let outcome = response.fight(opening);

            // score the fight, sum our pick with the outcome
            Ok(u32::from(outcome) + u32::from(response))
        })
        .collect();

    let score: u32 = scores?.iter().sum();

    println!("Score (per the guide): {score}");

    // Part 2 requires solving for the move that creates the given outcome and then calculating the score
    let scores: Result<Vec<u32>> = include_str!("input.txt")
        .lines()
        .map(|l| -> Result<u32> {
            let mut moves = l.split_whitespace();

            let opening = moves.next().ok_or(eyre!("No moves in line"))?.parse()?;
            let outcome = moves
                .next()
                .ok_or(eyre!("Only one move in line"))?
                .parse()?;

            let response = solve(opening, outcome);

            // score the fight, sum our pick with the outcome
            Ok(u32::from(outcome) + u32::from(response))
        })
        .collect();

    let score: u32 = scores?.iter().sum();

    println!("Scores (when read correctly): {score}");

    Ok(())
}

// Solve for what move gives the passed outcome from the passed opening
const fn solve(opening: Move, outcome: Outcome) -> Move {
    match outcome {
        Outcome::Win => match opening {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        Outcome::Loss => match opening {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        Outcome::Draw => opening,
    }
}
