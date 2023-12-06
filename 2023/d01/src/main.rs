use d01::sum_calibrations;

fn main() {
    let input = include_str!("input.txt");
    dbg!(sum_calibrations(input));
    let sum2: u32 = input
        .lines()
        .map(d01::tokenize)
        .map(|mut l| {
            let a = l.next().unwrap();
            let b = l.next_back().unwrap_or(a);
            a * 10 + b
        })
        .sum();

    dbg!(sum2);
}
