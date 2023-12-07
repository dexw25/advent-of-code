use core::str::Chars;
use std::collections::HashMap;

#[must_use]
pub fn sum_calibrations(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let num: Vec<u32> = l
                .chars()
                .filter(char::is_ascii_digit)
                .filter_map(|c| c.to_digit(10))
                .collect();

            num[0] * 10 + num[num.len() - 1]
        })
        .sum()
}

pub struct Tokenizer<'a> {
    source: Chars<'a>,
    token_map: HashMap<&'static str, u32>,
}

// This won't work for unicode. None of the below will
pub fn tokenize(input: &str) -> impl Iterator<Item = u32> + '_ + DoubleEndedIterator {
    Tokenizer {
        source: input.chars(),
        token_map: HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]),
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Borrow the full string for later
            let s = self.source.as_str();
            let c = self.source.next()?;

            if c.is_ascii_digit() {
                // this WILL be Some(...) because of the check above
                return c.to_digit(10);
            }
            for tok in self.token_map.keys() {
                if s.starts_with(tok) {
                    // seek the number of chars in the token, minus 1 for zero-indexing, minus 1 for the character already seeked at the top of this
                    self.source.nth(tok.len() - 2);
                    return self.token_map.get(tok).copied();
                }
            }
        }
    }
}

impl DoubleEndedIterator for Tokenizer<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            // Borrow the full string for later
            let s = self.source.as_str();
            let c = self.source.next_back()?;

            if c.is_ascii_digit() {
                // this WILL be Some(...) because of the check above
                break c.to_digit(10);
            }
            for tok in self.token_map.keys() {
                if s.ends_with(tok) {
                    // seek the number of chars in the token, minus 1 for zero-indexing, minus 1 for the character already seeked at the top of this
                    self.source.nth_back(tok.len() - 2);
                    return self.token_map.get(tok).copied();
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use super::sum_calibrations;
    use super::tokenize;

    #[test]
    fn sample1() {
        let input = include_str!("test.txt");
        assert_eq!(sum_calibrations(input), 142);
    }

    #[test]
    fn forward_tokenize() {
        assert_eq!(tokenize("two1nine").next(), Some(2));
        assert_eq!(tokenize("xtwone3four").next(), Some(2));
    }

    #[test]
    fn reverse_tokenize() {
        assert_eq!(tokenize("two1nine").next_back(), Some(9));
        assert_eq!(tokenize("xtwone3four").next_back(), Some(4));
        assert_eq!(tokenize("7pqrstsixteen").next_back(), Some(6));
    }

    #[test]
    fn sample2() {
        let input = include_str!("test2.txt").lines().map(tokenize);
        let sum: u32 = input
            .map(|mut l| {
                let a = l.next().unwrap();
                let b = l.next_back().unwrap_or(a);
                a * 10 + b
            })
            .sum();
        assert_eq!(sum, 281);
    }
}
