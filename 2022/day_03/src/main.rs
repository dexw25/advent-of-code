use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Default, Clone)]
struct RuckSack {
    pockets: (HashSet<Item>, HashSet<Item>),
}

mod item {
    #[derive(Debug, Hash, Clone, PartialEq, Eq, Copy)]
    pub struct Item {
        value: u8,
    }

    impl TryFrom<u8> for Item {
        type Error = ();

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'A'..=b'Z' => {
                    let val = value - b'A' + 27;

                    Ok(Self { value: val })
                }
                b'a'..=b'z' => {
                    let val = value - b'a' + 1;

                    Ok(Self { value: val })
                }
                _ => Err(()),
            }
        }
    }

    impl Item {
        pub(crate) const fn priority(self) -> u32 {
            self.value as u32
        }
    }
}

use item::Item;

#[allow(clippy::needless_range_loop)]
impl FromStr for RuckSack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sack = Self::default();

        let mut items = s
            .bytes()
            .map(Item::try_from)
            .collect::<Result<Vec<Item>, ()>>()?;

        let pouch_size = items.len() / 2;

        for _i in 0..pouch_size {
            sack.pockets.0.insert(items.pop().ok_or(())?);
        }
        for _i in 0..pouch_size {
            sack.pockets.1.insert(items.pop().ok_or(())?);
        }

        Ok(sack)
    }
}

fn main() -> Result<(), ()> {
    let sacks = include_str!("input.txt")
        .lines()
        .map(str::parse::<RuckSack>)
        .collect::<Result<Vec<_>, _>>()?;

    let part1: u32 = sacks
        .iter()
        .map(|s| {
            s.pockets
                .0
                .intersection(&s.pockets.1)
                .copied()
                .map(item::Item::priority)
                .sum::<u32>()
        })
        .sum();

    println!("p1: {part1}");

    // Iterate over chunks of 3
    #[allow(clippy::expect_used)]
    let sum: u32 = sacks
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|sack| {
                    let mut s: HashSet<Item> = HashSet::new();
                    s.extend(&sack.pockets.0);
                    s.extend(&sack.pockets.1);
                    s
                })
                .reduce(|mut a, s| {
                    a.retain(|e| s.contains(e));
                    a
                })
                .expect("no input?")
                .iter()
                .next()
                .expect("No common items?")
                .priority()
        })
        .sum();

    println!("p2: {sum:?}");
    Ok(())
}
