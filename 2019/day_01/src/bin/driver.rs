use std::fs;
use day_01::{part_one, part_two};

fn main() -> std::io::Result<()> {
    let buf = &fs::read("./input.txt")?;
    let parts: Vec<u32> = std::str::from_utf8(buf)
        .unwrap()
        .trim()
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("Part 1 Solution: {}", part_one(&parts));
    println!("Part 2 Solution: {}", part_two(&parts));
    Ok(())
}