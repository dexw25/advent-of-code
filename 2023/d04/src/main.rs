use d04::Card;

fn main() {
    let input = include_str!("input.txt");
    let sum = input
        .lines()
        .map(|l| l.parse::<Card>().unwrap().score())
        .sum::<u32>();

    dbg!(sum);
}
