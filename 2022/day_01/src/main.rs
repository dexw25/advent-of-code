use std::error::Error;

struct Elf {
    foods: Vec<u32>,
}

fn parse_input(input: &str) -> Result<Vec<Elf>, Box<dyn Error>> {
    let mut elves: Vec<Elf> = Vec::new();
    let _ = input
        .lines()
        .map(|row| {
            if row.is_empty() {
                elves.push(Elf { foods: Vec::new() });
            } else if let Some(elf) = elves.last_mut() {
                elf.foods.push(row.parse()?);
            } else {
                // catchall in case elves is empty
                elves.push(Elf {
                    foods: vec![row.parse()?],
                });
            }
            Ok(())
        })
        .collect::<Result<Vec<()>, Box<dyn Error>>>()?;

    Ok(elves)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_input(include_str!("input.txt"))?;

    println!("parsed {} elves", input.len());

    // Calculate the sum of each elves stash and keep in a list
    let mut sums: Vec<u32> = input.iter().map(|elf| elf.foods.iter().sum()).collect();

    // No, I'm not writing a sorting algorithm
    sums.sort_unstable();

    // part 1
    println!("Biggest stash: {}", sums.last().ok_or("unreachable")?);

    // part 2
    let l = sums.len();
    println!(
        "Biggest 3 together: {}",
        sums[l - 1] + sums[l - 2] + sums[l - 3]
    );

    Ok(())
}
