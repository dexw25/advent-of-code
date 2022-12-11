use std::{
    collections::{hash_set::Union, HashSet},
    hash::BuildHasher,
    str::FromStr,
};

#[derive(Debug, Default, Clone)]
struct RuckSack {
    pockets: [HashSet<Item>; 2],
}

impl RuckSack {
    fn all_items<'a, S: BuildHasher>(&'a self) -> Union<'a, Item, S> {
        self.pockets[0].union(&self.pockets[1])
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Copy)]
struct Item {
    priority: u32,
}

impl TryFrom<char> for Item {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A'..='Z' => {
                let val = value as u32 - 'A' as u32 + 27;

                Ok(Self { priority: val })
            }
            'a'..='z' => {
                let val = value as u32 - 'a' as u32 + 1;

                Ok(Self { priority: val })
            }
            _ => Err(()),
        }
    }
}

#[allow(clippy::needless_range_loop)]
impl FromStr for RuckSack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sack = Self::default();

        let items = s
            .chars()
            .map(Item::try_from)
            .collect::<Result<Vec<Item>, ()>>()?;

        let pouch_size = items.len() / 2;

        for i in 0..pouch_size {
            sack.pockets[0].insert(items[i]);
        }
        for i in 0..pouch_size {
            sack.pockets[1].insert(items[i + pouch_size]);
        }

        Ok(sack)
    }
}

fn main() -> Result<(), ()> {
    let sacks: Result<Vec<RuckSack>, ()> =
        include_str!("input.txt").lines().map(str::parse).collect();

    let part1: u32 = sacks?
        .iter()
        .map(|s| {
            s.pockets[0]
                .intersection(&s.pockets[1])
                .map(|i| i.priority)
                .sum::<u32>()
        })
        .sum();

    println!("p1: {part1}");

    let i = sacks?.iter();
    let mut sum: u32 = 0;

    // Loop through, iterating through groups of 3
    whie let Some(s_1) = i.next() {
        let s_2 = i.next().ok_or(())?;
        let s_3 = i.next().ok_or(())?;

    };

    println!("p2: {sum}");
    Ok(())
}
