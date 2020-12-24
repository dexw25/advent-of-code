mod data;
use crate::data::DATA;
use regex::Regex;
use std::iter::FromIterator;
use std::str::FromStr;

// Represent expression as tree
#[derive(Debug)]
enum Expression {
    Constant(u64),
    OperatorAdd(Box<Expression>, Box<Expression>), // lhs + rhs
    OperatorMul(Box<Expression>, Box<Expression>), // lhs * rhs
}

#[derive(Debug)]
enum Token {
    Constant(u64),
    Add,
    Mul,
    RParen,
    LParen,
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

impl<'a> Expression {
    fn evaluate(&self) -> u64 {
        use Expression::*;
        match self {
            Constant(n) => *n,
            OperatorAdd(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            OperatorMul(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
        }
    }

    // 1 + (2 * 3) + (4 * (5 + 6))
    // Given iterator of remaining tokens, join the passed token to whatever's to the left of it
    fn recursive_ast<T: Iterator<Item = &'a Token>>(
        i: &mut T,
        right: Option<Box<Expression>>,
    ) -> Box<Expression> {
        use Token::*;

        // Iterate over all tokens
        match i.next() {
            // pass constant to stick on next token
            Some(Constant(num)) => {
                Expression::recursive_ast(i, Some(Box::new(Expression::Constant(*num))))
            }
            // create node with rhs and whatever is to the left of this token
            Some(Add) => Box::new(Expression::OperatorAdd(
                Expression::recursive_ast(i, None),
                right.unwrap(),
            )),
            // See add
            Some(Mul) => Box::new(Expression::OperatorMul(
                Expression::recursive_ast(i, None),
                right.unwrap(),
            )),
            Some(RParen) => {
                // Parse entire subexpression, pass as right hand side to next op
                let subexpr = Expression::recursive_ast(i, None);
                Expression::recursive_ast(i, Some(subexpr))
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

impl FromStr for Expression {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // tokenize and pass iterator of tokens to tree builder
        let r = Regex::new(r"([()+*]|[\d]+)").unwrap();
        let tokens: Vec<Token> = r
            .find_iter(s)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let ret = tokens.iter().rev().collect(); // reverse tokens and build the tree
        println!("{:#?}", ret);
        Ok(ret)
    }
}
// Recursively parse a series of tokens into an AST of sorts
impl<'a> FromIterator<&'a Token> for Expression {
    fn from_iter<T: IntoIterator<Item = &'a Token>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        *Expression::recursive_ast(&mut i, None)
    }
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.parse::<Expression>().unwrap().evaluate())
        .sum()
}

fn main() {
    println!("{}", part_1(DATA));
}
//

#[test]
fn math_test_00() {
    let input = "1 + (2 * 3) + (4 * (5 + 6))";

    assert_eq!(input.parse::<Expression>().unwrap().evaluate(), 51);
}
#[test]
fn math_test_0() {
    let input = "1 + 2 * 3 + 4 * 5 + 6";

    assert_eq!(input.parse::<Expression>().unwrap().evaluate(), 71);
}
#[test]
fn math_test_1() {
    let input = "2 * 3 + (4 * 5)";

    assert_eq!(input.parse::<Expression>().unwrap().evaluate(), 26);
}
#[test]
fn math_test_2() {
    let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";

    assert_eq!(input.parse::<Expression>().unwrap().evaluate(), 437);
}
#[test]
fn math_test_3() {
    let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";

    assert_eq!(input.parse::<Expression>().unwrap().evaluate(), 12240);
}

#[test]
fn math_test_4() {
    let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    assert_eq!(input.parse::<Expression>().unwrap().evaluate(), 13632);
}
