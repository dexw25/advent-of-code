use d01::sum_calibrations;

fn main() -> Result<(), &'static str> {
    let input = include_str!("input.txt");
    dbg!(sum_calibrations(input));
    let sum2: Result<Vec<u32>, _> = input
        .lines()
        .map(d01::tokenize)
        .map(|mut l| {
            let a = l.next().ok_or("Empty iter?")?;
            let b = l.next_back().unwrap_or(a);
            Ok::<u32, &'static str>(a * 10 + b)
        })
        .collect();

    dbg!(sum2?.iter().sum::<u32>());

    Ok(())
}
