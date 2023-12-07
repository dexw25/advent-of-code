use core::str::FromStr;
use std::collections::HashSet;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Card {
    _id: usize,
    have: HashSet<u64>,
    winning: HashSet<u64>,
}

impl Card {
    #[must_use]
    pub fn score(&self) -> u64 {
        let total = self.winning.intersection(&self.have).count();
        if total == 0 {
            0
        } else {
            1 << (total - 1)
        }
    }

    #[must_use]
    pub fn num_wins(&self) -> usize {
        self.winning.intersection(&self.have).count()
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

        Ok(Self {
            _id: id,
            have,
            winning,
        })
    }
}

pub struct CardsP2<T> {
    offsets: VecDeque<u64>,
    cards: T,
}

pub fn part_two<T: Iterator<Item = Card>>(cards: T) -> CardsP2<T> {
    CardsP2 {
        offsets: VecDeque::new(),
        cards,
    }
}

impl<T: Iterator<Item = Card>> Iterator for CardsP2<T> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.cards.next()?;
        dbg!(&self.offsets);
        // dbg!(&c);
        let cards_inhand = match self.offsets.pop_front() {
            Some(o) => o + 1,
            None => 1,
        };

        let wins = c.num_wins();
        let mut i = 0;
        while i < wins {
            if let Some(o) = self.offsets.get_mut(i) {
                *o += cards_inhand;
            } else {
                self.offsets.push_back(cards_inhand);
            }

            i += 1;
        }
        // dbg!(wins, cards_inhand);
        Some(cards_inhand)
    }
}
