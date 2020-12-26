mod data;
use data::DATA;

use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

// Rule types to parse from input
#[derive(Debug, Eq, PartialEq)]
enum Rule {
    And(Vec<Self>),           // Match all in series (arbitrarily many)
    Or(Box<Self>, Box<Self>), // Match either one rule or the other
    Char(char),               // Match one character
    Ref(usize),               // Reference an existing rule
}

impl FromStr for Rule {
    type Err = ();

    // '1 3 | 3 1' or '"b"' or '1 2'
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.contains('|') {
            // Recurse for OR case
            let mut iter = s.split('|').map(|r| r.trim().parse::<Self>().unwrap());
            Self::Or(
                Box::new(iter.next().unwrap()),
                Box::new(iter.next().unwrap()),
            )
        } else if s.contains('"') {
            // Simplest character match
            Self::Char(s.chars().find(|c| char::is_alphabetic(*c)).unwrap())
        } else {
            // Basic AND case, only support references here
            Self::And(
                s.split(' ')
                    .map(|v| Self::Ref(v.parse::<usize>().unwrap()))
                    .collect(),
            )
        })
    }
}

impl Rule {
    // Simply recursively walk the AST and resolve each piece to a regular expression syntax
    fn to_regex(&self, rules: &HashMap<usize, Rule>) -> String {
        match self {
            Rule::And(r) => r.iter().map(|r| r.to_regex(rules)).collect(),
            Rule::Or(left, right) => {
                format!("({}|{})", left.to_regex(rules), right.to_regex(rules))
            }
            Rule::Char(c) => c.to_string(),
            Rule::Ref(n) => rules.get(n).unwrap().to_regex(rules),
        }
    }

    // like above but patch 2 rules with some special regexes
    //8: 42 becomes 8: 42 | 42 8, or (42)+
    //11: 42 31 becomes 11: 42 31 | 42 11 31, or match the same number of instances of 42 then 31
    fn to_regexv2(&self, rules: &HashMap<usize, Rule>) -> String {
        match self {
            Rule::And(r) => {
                // Check for either of the patched rules and patch them inline
                if *r == [Self::Ref(42)] {
                    // Simple one-or-more wildcard
                    format!("({})+", Self::Ref(42).to_regexv2(rules))
                } else if *r == [Self::Ref(42), Self::Ref(31)] {
                    // Hack it by generating a number of possible combinations and just generating expressions for each
                    format!(
                        "({})",
                        (1..10) // 10 chosen here arbitrarily because it passes on test input
                            .map(|n| {
                                format!(
                                    "(({}){{{}}}({}){{{}}})",
                                    Self::Ref(42).to_regexv2(rules),
                                    n,
                                    Self::Ref(31).to_regexv2(rules),
                                    n
                                )
                            })
                            .collect::<Vec<String>>()
                            .join("|")
                    )
                } else {
                    r.iter().map(|r| r.to_regexv2(rules)).collect()
                }
            }
            Rule::Or(left, right) => {
                format!("({}|{})", left.to_regexv2(rules), right.to_regexv2(rules))
            }
            Rule::Char(c) => c.to_string(),
            Rule::Ref(n) => rules.get(n).unwrap().to_regexv2(rules),
        }
    }
}

fn parse_all_rules(s: &str) -> HashMap<usize, Rule> {
    let mut ret = HashMap::new();

    for line in s.lines() {
        let mut elements = line.split(": ");
        let idx = elements.next().unwrap().parse::<usize>().unwrap();
        let rule = elements.next().unwrap().parse::<Rule>().unwrap();
        ret.insert(idx, rule);
    }

    ret
}

fn part_1(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let rule_tree = parse_all_rules(&parts.next().unwrap());
    let expr = Regex::new(&format!(
        "^{}$",
        rule_tree.get(&0).unwrap().to_regex(&rule_tree)
    ))
    .unwrap();

    parts
        .next()
        .unwrap()
        .lines()
        .filter(|l| expr.is_match(l))
        .count()
}

// Like part 1, but patch 2 rules to add loops
fn part_2(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let rule_tree = parse_all_rules(&parts.next().unwrap());

    let expr = Regex::new(&format!(
        "^{}$",
        rule_tree.get(&0).unwrap().to_regexv2(&rule_tree)
    ))
    .unwrap();

    parts
        .next()
        .unwrap()
        .lines()
        .filter(|l| expr.is_match(l))
        .count()
}

fn main() {
    println!("p1: {:#?}", part_1(DATA));
    println!("p2: {:#?}", part_2(DATA));
}

#[test]
fn test_1() {
    let input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    assert_eq!(part_1(input), 2);
}
#[test]
fn test_2() {
    let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    assert_eq!(part_1(input), 3);
}

#[test]
fn test_part2() {
    let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    assert_eq!(part_2(input), 12);
}
