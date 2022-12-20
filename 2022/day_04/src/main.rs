use color_eyre::{eyre::eyre, Result};

mod section {
    use crate::eyre;
    use std::{fmt::Display, str::FromStr};

    #[derive(Debug, Clone)]
    pub struct Assignment {
        range: std::ops::RangeInclusive<usize>,
    }

    impl FromStr for Assignment {
        type Err = color_eyre::Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // Iterate over input, parsing the first 2 successful number items, discarding segments that fail
            let mut nums = s.split('-').map(str::parse).into_iter().flatten();

            Ok(Self {
                range: nums.next().ok_or(eyre!("Not enough numbers in input"))?
                    ..=nums.next().ok_or(eyre!("Not enough numbers in input"))?,
            })
        }
    }

    impl Display for Assignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}-{}", self.range.start(), self.range.end())
        }
    }

    impl Assignment {
        pub fn contains(&self, other: &Self) -> bool {
            self.range.contains(other.range.start()) && self.range.contains(other.range.end())
                || other.range.contains(self.range.start())
                    && other.range.contains(self.range.end())
        }

        // Another way of stating overlaps is if the other assignment contains the start or end of this one
        pub fn overlaps(&self, other: &Self) -> bool {
            self.range.contains(other.range.start())
                || self.range.contains(other.range.end())
                // Make sure to include complete containment, as this also counts for overlap
                || self.contains(other)
        }
    }
}

use section::Assignment;

fn parse_input(input: &str) -> Result<Vec<(Assignment, Assignment)>> {
    input
        .lines()
        .map(|l| -> Result<(Assignment, Assignment)> {
            let mut i = l.split(',').map(|s| {
                s.parse::<Assignment>()
                    .map_err(|e| eyre!("Error {e} in parsing"))
            });
            let first = i.next().ok_or(eyre!("No input?"))??;
            let second = i.next().ok_or(eyre!("No input?"))??;

            Ok((first, second))
        })
        .collect()
}

#[allow(clippy::unwrap_used, clippy::unnecessary_wraps)]
fn main() -> color_eyre::Result<()> {
    let input = include_str!("input.txt");

    let data = parse_input(input)?;

    let day_02p1 = data.iter().filter(|pair| pair.0.contains(&pair.1)).count();

    dbg!(day_02p1);

    let day_02p2 = data.iter().filter(|pair| pair.0.overlaps(&pair.1)).count();
    dbg!(day_02p2);

    Ok(())
}
