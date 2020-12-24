mod data;
use crate::data::DATA;
use regex::Regex;
use std::iter::FromIterator;
use std::str::FromStr;

// Represent expression as tree
#[derive(Debug)]
enum ExpressionAST {
    Constant(u64),
    OperatorAdd(Box<Self>, Box<Self>), // lhs + rhs
    OperatorMul(Box<Self>, Box<Self>), // lhs * rhs
}

// Order defines operator precedence by bottom-to-top order
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Ord)]
enum Token {
    Constant(u64),
    RParen,
    LParen,
    Mul,
    Add,
}

impl FromStr for Token {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<u64>() {
            Ok(Token::Constant(num))
        } else {
            match s {
                "+" => Ok(Token::Add),
                "*" => Ok(Token::Mul),
                "(" => Ok(Token::LParen),
                ")" => Ok(Token::RParen),
                _ => Err(()),
            }
        }
    }
}

impl<'a> ExpressionAST {
    fn evaluate(&self) -> u64 {
        match self {
            Self::Constant(n) => *n,
            Self::OperatorAdd(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Self::OperatorMul(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
        }
    }

    // 1 + (2 * 3) + (4 * (5 + 6))
    // Given iterator of remaining tokens, join the passed token to whatever's to the left of it
    fn recursive_ast<T: Iterator<Item = &'a Token>>(
        i: &mut T,
        right: Option<Box<Self>>,
    ) -> Box<Self> {
        use Token::*;

        // Iterate over all tokens
        match i.next() {
            // pass constant to stick on next token
            Some(Constant(num)) => Self::recursive_ast(i, Some(Box::new(Self::Constant(*num)))),
            // create node with rhs and whatever is to the left of this token
            Some(Add) => Box::new(Self::OperatorAdd(
                Self::recursive_ast(i, None),
                right.unwrap(),
            )),
            // See add
            Some(Mul) => Box::new(Self::OperatorMul(
                Self::recursive_ast(i, None),
                right.unwrap(),
            )),
            Some(RParen) => {
                // Parse entire subexpression, pass as right hand side to next op
                let subexpr = Self::recursive_ast(i, None);
                Self::recursive_ast(i, Some(subexpr))
            }
            // End recursion with whatever right token was passed
            Some(LParen) => right.unwrap(),
            None => {
                // see LParen
                right.unwrap()
            }
        }
    }
}

impl FromStr for ExpressionAST {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // tokenize and pass iterator of tokens to tree builder
        let r = Regex::new(r"([()+*]|[\d]+)").unwrap();
        let tokens: Vec<Token> = r
            .find_iter(s)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let ret = tokens.iter().rev().collect(); // reverse tokens and build the tree
        Ok(ret)
    }
}

// Recursively parse a series of tokens into an AST of sorts
impl<'a> FromIterator<&'a Token> for ExpressionAST {
    fn from_iter<T: IntoIterator<Item = &'a Token>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        *Self::recursive_ast(&mut i, None)
    }
}

// RPN representation with wierd operator precedence
type ExpressionRPN = Vec<Token>;

fn evaluate(rpn: ExpressionRPN) -> u64 {
    use Token::*;
    let mut data: Vec<Token> = Vec::new();

    for t in rpn.iter() {
        match t {
            Mul => {
                if let (Some(Constant(lhs)), Some(Constant(rhs))) = (data.pop(), data.pop()) {
                    data.push(Constant(lhs * rhs));
                } else {
                    panic!("No data for mul");
                }
            }
            Add => {
                if let (Some(Constant(lhs)), Some(Constant(rhs))) = (data.pop(), data.pop()) {
                    data.push(Constant(lhs + rhs));
                } else {
                    panic!("No data for add");
                }
            }
            val => data.push(*val),
        }
    }

    match data.pop() {
        Some(Constant(n)) => n,
        _ => 0,
    }
}

// Instead of a recursive AST implement the Shunting-Yard algorithm and build an RPN stack
//1 + (2 * 3) + (4 * (5 + 6))
fn shunting_yard(s: &str) -> ExpressionRPN {
    use Token::*;
    // tokenize and handoff tokens to shunting-yard
    let r = Regex::new(r"([()+*]|[\d]+)").unwrap();

    let input = r.find_iter(s).map(|m| m.as_str().parse::<Token>().unwrap()); // Initialize token list from input string

    let mut operators = Vec::new();
    let mut output = Vec::new();

    // Build RPN stack
    for t in input {
        match t {
            Constant(_) => {
                output.push(t);
            }
            LParen => {
                operators.push(t);
            }
            Add | Mul => {
                while let Some(op) = operators.last() {
                    if op >= &t && op != &LParen {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(t);
            }
            RParen => {
                // push everything that isn't a leftparen onto the output stack
                while let Some(op) = operators.last() {
                    if op != &LParen {
                        output.push(operators.pop().unwrap())
                    } else {
                        operators.pop();
                        // Discard lparen and break
                        break;
                    }
                }
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    output
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.parse::<ExpressionAST>().unwrap().evaluate())
        .sum()
}

fn part_2(input: &str) -> u64 {
    input.lines().map(|l| evaluate(shunting_yard(l))).sum()
}

fn main() {
    println!("{}", part_1(DATA));
    println!("{}", part_2(DATA));
}

#[test]
fn math_test_00() {
    let input = "1 + (2 * 3) + (4 * (5 + 6))";

    assert_eq!(input.parse::<ExpressionAST>().unwrap().evaluate(), 51);
    assert_eq!(part_2(input), 51);
}
#[test]
fn math_test_0() {
    let input = "1 + 2 * 3 + 4 * 5 + 6";

    assert_eq!(input.parse::<ExpressionAST>().unwrap().evaluate(), 71);
    assert_eq!(part_2(input), 231);
}
#[test]
fn math_test_1() {
    let input = "2 * 3 + (4 * 5)";

    assert_eq!(input.parse::<ExpressionAST>().unwrap().evaluate(), 26);
}
#[test]
fn math_test_2() {
    let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(input.parse::<ExpressionAST>().unwrap().evaluate(), 437);
    assert_eq!(part_2(input), 1445);
}
#[test]
fn math_test_3() {
    let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";

    assert_eq!(input.parse::<ExpressionAST>().unwrap().evaluate(), 12240);
    assert_eq!(part_2(input), 669060);
}

#[test]
fn math_test_4() {
    let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    assert_eq!(input.parse::<ExpressionAST>().unwrap().evaluate(), 13632);
    assert_eq!(part_2(input), 23340);
}
