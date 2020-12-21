mod data;
use crate::data::DATA;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(PartialEq, Hash, Eq)]
struct Rule {
    label: String,
    lower_range: RangeInclusive<u32>,
    upper_range: RangeInclusive<u32>,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split(": ");
        let label = fields.next().unwrap().trim();

        // Parse fields with splitting
        let ranges: Vec<Vec<u32>> = fields
            .next()
            .unwrap()
            .split("or")
            .map(|s| {
                s.trim()
                    .split('-')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        Ok(Rule {
            label: label.to_string(),
            lower_range: (ranges[0][0]..=ranges[0][1]),
            upper_range: (ranges[1][0]..=ranges[1][1]),
        })
    }
}

impl Rule {
    fn validate(&self, val: u32) -> bool {
        self.lower_range.contains(&val) || self.upper_range.contains(&val)
    }
}

fn part_1(input: &str) -> u32 {
    let split_input: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<Rule> = split_input[0]
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let _your_ticket = split_input[1]
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();

    // Parse and validate all fields of nearby tickets in a concise manner
    let invalid_sum = split_input[2]
        .lines()
        .skip(1)
        .flat_map(|l| l.split(','))
        .map(|s| s.parse::<u32>().unwrap())
        .filter(|n| !rules.iter().map(|r| r.validate(*n)).any(|x| x))
        .sum::<u32>();

    invalid_sum
}

// Toss invalid tickets and deduce the order of the fields, then multiply all fields on my ticket that start with "departure"
fn part_2(input: &str) -> u64 {
    let split_input: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<Rule> = split_input[0]
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    // Parse my ticket
    let your_ticket = split_input[1]
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Parse and throw away invalid tickets
    let valid_tickets = split_input[2]
        .lines()
        .skip(1)
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|t| {
            t.iter()
                // Filter ticket for which all elements pass any rule
                .all(|x| rules.iter().map(|r| r.validate(*x)).any(|y| y))
        })
        .collect::<Vec<_>>(); // collect because we will be iterating on this quite a bit

    // Build a map of where we know a given rule _could_ go
    let mut unknown_rules: HashMap<&Rule, HashSet<usize>> = HashMap::new();

    // Iterate rules, and build a map of columns for which all values satisfy the constraint
    rules.iter().for_each(|r| {
        unknown_rules.insert(
            r,
            (0..valid_tickets[0].len())
                .filter(|i| valid_tickets.iter().all(|t| r.validate(t[*i])))
                .collect(),
        );
    });

    // Store known rules in a list
    let mut known_rules = HashMap::new();

    // iterate over unknown rules and insert into known_rules
    loop {
        // For rules which have only one possible column, drain them and insert into known_rules
        let r = unknown_rules
            .clone()
            .into_iter()
            .find(|u| u.1.len() == 1)
            .unwrap();
        let num = *r.1.iter().next().unwrap();
        known_rules.insert(r.0, num);
        unknown_rules.remove(r.0);
        for v in unknown_rules.values_mut() {
            v.remove(&num);
        }

        if unknown_rules.is_empty() {
            break;
        }
    }

    // Calculate the actual answer
    known_rules
        .iter()
        .filter_map(|t| {
            if t.0.label.starts_with("departure") {
                Some(your_ticket[*t.1])
            } else {
                None
            }
        })
        .product()
}

fn main() {
    println!("P1: {}", part_1(DATA));
    println!("P2: {}", part_2(DATA));
}

#[test]
fn validate_1() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    assert_eq!(part_1(input), 71);
}
