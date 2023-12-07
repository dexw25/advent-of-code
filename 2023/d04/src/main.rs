use d04::{part_two, Card};

fn main() {
    let input = include_str!("input.txt");
    let sum = input
        .lines()
        .map(|l| l.parse::<Card>().unwrap().score())
        .sum::<u64>();

    dbg!(sum);

    let deck = input.lines().map(|l| l.parse::<Card>().unwrap());
    let p2 = part_two(deck);
    // for i in p2 {
    // dbg!(i);
    // }

    dbg!(p2.sum::<u64>());
}
