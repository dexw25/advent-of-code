use core::str::FromStr;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Card {
    id: usize,
    have: HashSet<u32>,
    winning: HashSet<u32>,
}

impl Card {
    #[must_use]
    pub fn score(&self) -> u32 {
        let total = self.winning.intersection(&self.have).count();
        if total == 0 {
            0
        } else {
            1 << (total - 1)
        }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(':');
        let id = s
            .next()
            .ok_or(())?
            .split_whitespace()
            .nth(1)
            .ok_or(())?
            .parse::<usize>()
            .map_err(|_| ())?;

        let mut rhs = s.next().ok_or(())?.split('|');

        let haveres: Result<_, _> = rhs
            .next()
            .ok_or(())?
            .split_whitespace()
            .map(|s| s.parse().map_err(|_| ()))
            .collect();
        let have = haveres?;

        let winningres: Result<_, _> = rhs
            .next()
            .ok_or(())?
            .split_whitespace()
            .map(|s| s.parse().map_err(|_| ()))
            .collect();
        let winning = winningres?;

        Ok(Self { id, have, winning })
    }
}
