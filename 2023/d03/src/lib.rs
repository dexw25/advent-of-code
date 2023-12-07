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
}

pub struct PartNumberIter<'a, T: Iterator<Item = &'a &'a str>> {
    sch: &'a Schematic<'a>,
    row_number: usize,
    lines: T,
    current_line: &'a str,
    line_cursor: CharIndices<'a>,
}

impl<'a, T: Iterator<Item = &'a &'a str>> Iterator for PartNumberIter<'a, T> {
    type Item = u32;
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
                        let num: u32 = num.parse().ok()?;

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
}
