use core::{slice::Iter, str::CharIndices};

pub struct Schematic<'a> {
    // 2D array to store the schematic, list of refs to lines
    schematic: Vec<&'a str>,
}

impl<'a> Schematic<'a> {
    #[must_use]
    pub fn new(s: &'a str) -> Self {
        Self {
            schematic: s.lines().collect(),
        }
    }

    #[must_use]
    pub fn part_numbers_iter(&self) -> Option<PartNumberIter<Iter<&str>>> {
        let mut lines = self.schematic.iter();
        let current_line = lines.next()?;
        let line_cursor = current_line.char_indices();
        Some(PartNumberIter {
            sch: self,
            lines,
            current_line,
            line_cursor,
            row_number: 0,
        })
    }

    /// # Panics
    /// - If the input is empty
    #[must_use]
    #[allow(clippy::unwrap_used)]
    pub fn gear_ratios_iter(&self) -> GearRatioIter<Iter<&str>> {
        let mut lines = self.schematic.iter();
        let current_line = lines.next().unwrap();
        let line_cursor = current_line.char_indices();
        GearRatioIter {
            sch: self,
            lines,
            current_line,
            line_cursor,
            row_number: 0,
        }
    }

    #[allow(clippy::unwrap_used)]
    fn is_part_number(&self, row: usize, col: usize, len: usize) -> bool {
        let mut boundary = String::new();

        // Row above
        if row >= 1 {
            let end = if (col + len) < self.schematic[row - 1].len() {
                col + len
            } else {
                col + len - 1
            };
            boundary.push_str(&self.schematic[row - 1][col.saturating_sub(1)..=(end)]);
        }

        if col > 0 {
            boundary.push(
                self.schematic[row]
                    .chars()
                    .nth(col.saturating_sub(1))
                    .unwrap(),
            );
        }

        if (col + len) < self.schematic[row].len() {
            boundary.push(self.schematic[row].chars().nth(col + len).unwrap());
        }

        if row < (self.schematic.len() - 1) {
            let end = if (col + len) < self.schematic[row + 1].len() {
                col + len
            } else {
                col + len - 1
            };
            boundary.push_str(&self.schematic[row + 1][col.saturating_sub(1)..=end]);
        }

        for c in boundary.chars() {
            if c != '.' {
                return true;
            }
        }

        false
    }

    fn calculate_ratio(&self, row: usize, col: usize) -> Option<u64> {
        let mut boundary = vec![];

        // Row above
        if row >= 1 {
            let mut start = col - 1;
            let mut end = if (col + 1) < self.schematic[row - 1].len() {
                col + 1
            } else {
                col
            };

            if self.schematic[row - 1]
                .chars()
                .nth(start)
                .unwrap()
                .is_ascii_digit()
            {
                while start > 0
                    && self.schematic[row - 1]
                        .chars()
                        .nth(start - 1)
                        .unwrap()
                        .is_ascii_digit()
                {
                    start -= 1;
                }
            }
            if self.schematic[row - 1]
                .chars()
                .nth(end)
                .unwrap()
                .is_ascii_digit()
            {
                while end < self.schematic[row - 1].len() - 1
                    && self.schematic[row - 1]
                        .chars()
                        .nth(end + 1)
                        .unwrap()
                        .is_ascii_digit()
                {
                    end += 1;
                }
            }

            boundary.push(&self.schematic[row - 1][start..=end]);
        }

        if col > 0 {
            let mut start = col - 1;
            let mut end = col;
            if self.schematic[row]
                .chars()
                .nth(start)
                .unwrap()
                .is_ascii_digit()
            {
                while start > 0
                    && self.schematic[row]
                        .chars()
                        .nth(start - 1)
                        .unwrap()
                        .is_ascii_digit()
                {
                    start -= 1;
                }
            }
            boundary.push(&self.schematic[row][start..end]);
        }

        if (col + 1) < self.schematic[row].len() {
            let mut start = col + 1;
            let mut end = col + 2;
            if self.schematic[row]
                .chars()
                .nth(start)
                .unwrap()
                .is_ascii_digit()
            {
                while end < self.schematic[row].len() - 1
                    && self.schematic[row]
                        .chars()
                        .nth(end)
                        .unwrap()
                        .is_ascii_digit()
                {
                    end += 1;
                }
            }
            boundary.push(&self.schematic[row][start..end]);
        }

        if row < (self.schematic.len() - 1) {
            let mut start = col - 1;
            let mut end = if (col + 1) < self.schematic[row + 1].len() {
                col + 1
            } else {
                col
            };

            if self.schematic[row + 1]
                .chars()
                .nth(start)
                .unwrap()
                .is_ascii_digit()
            {
                while start > 0
                    && self.schematic[row + 1]
                        .chars()
                        .nth(start - 1)
                        .unwrap()
                        .is_ascii_digit()
                {
                    start -= 1
                }
            }
            if self.schematic[row + 1]
                .chars()
                .nth(end)
                .unwrap()
                .is_ascii_digit()
            {
                while end < self.schematic[row + 1].len() - 1
                    && self.schematic[row + 1]
                        .chars()
                        .nth(end + 1)
                        .unwrap()
                        .is_ascii_digit()
                {
                    end += 1;
                }
            }
            boundary.push(&self.schematic[row + 1][start..=end]);
        }

        let mut delimited_boundary = String::new();
        let mut b_iter = boundary.iter();
        delimited_boundary.push_str(b_iter.next().unwrap());
        for b in b_iter {
            delimited_boundary.push('.');
            delimited_boundary.push_str(b);
        }

        let nums: Vec<u64> = delimited_boundary
            .split('.')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        if nums.len() == 2 {
            Some(nums.iter().product())
        } else {
            None
        }
    }
}

pub struct PartNumberIter<'a, T: Iterator<Item = &'a &'a str>> {
    sch: &'a Schematic<'a>,
    row_number: usize,
    lines: T,
    current_line: &'a str,
    line_cursor: CharIndices<'a>,
}

impl<'a, T: Iterator<Item = &'a &'a str>> Iterator for PartNumberIter<'a, T> {
    type Item = u64;
    #[allow(clippy::while_let_loop)]
    fn next(&mut self) -> Option<Self::Item> {
        // iterate lines
        loop {
            // iterate chars
            loop {
                if let Some((i, c)) = self.line_cursor.next() {
                    if c.is_ascii_digit() {
                        let mut num = String::from(c);

                        for (_, c) in self.line_cursor.by_ref() {
                            if c.is_ascii_digit() {
                                num.push(c);
                            } else {
                                break;
                            }
                        }

                        let num_length = num.len();
                        // Should be pretty infallible
                        let num: u64 = num.parse().ok()?;

                        // check if the number is a part number against the schematic
                        if self.sch.is_part_number(self.row_number, i, num_length) {
                            return Some(num);
                        }
                    } // implicit else for these if's... continue loop iteration to next digit
                } else {
                    break;
                }
            }

            self.current_line = self.lines.next()?;
            self.row_number += 1;
            self.line_cursor = self.current_line.char_indices();
        }
    }
}

pub struct GearRatioIter<'a, T: Iterator<Item = &'a &'a str>> {
    sch: &'a Schematic<'a>,
    lines: T,
    current_line: &'a str,
    line_cursor: CharIndices<'a>,
    row_number: usize,
}

impl<'a, T: Iterator<Item = &'a &'a str>> Iterator for GearRatioIter<'a, T> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // iterate lines
        loop {
            // iterate chars
            for (i, c) in self.line_cursor.by_ref() {
                if c == '*' {
                    if let Some(ratio) = self.sch.calculate_ratio(self.row_number, i) {
                        return Some(ratio);
                    }
                }
            }
            self.current_line = self.lines.next()?;
            self.row_number += 1;
            self.line_cursor = self.current_line.char_indices();
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::Schematic;

    #[test]
    fn parse_part_numbers() {
        let input = include_str!("test.txt");

        let sch = Schematic::new(input);
        let mut i = sch.part_numbers_iter().unwrap();

        assert_eq!(i.next(), Some(467));
        assert_eq!(i.next(), Some(35));
        assert_eq!(i.next(), Some(633));
        assert_eq!(i.next(), Some(617));
        assert_eq!(i.next(), Some(592));
        assert_eq!(i.next(), Some(755));
        assert_eq!(i.next(), Some(664));
        assert_eq!(i.next(), Some(598));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn parse_gear_ratios() {
        let input = include_str!("test.txt");

        let sch = Schematic::new(input);
        let mut i = sch.gear_ratios_iter();
        assert_eq!(i.next(), Some(16345));
        assert_eq!(i.next(), Some(451_490));
        assert_eq!(i.next(), None);
    }
}
