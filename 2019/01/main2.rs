// Read hardcoded path: input.txt and calculate the toal fuel need
// Each line is a moudle weight, implement the following algorithim:
//  "to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2"

// Part two has an iterative problem, fuel must now be accounted for

use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
	let mut total_fuel = 0;

	// Open file
	let file = File::open("./input.txt")?;
	let file = BufReader::new(file);

	// iterate over lines and attempt to parse as int
	for line in file.lines() {
		let mass = line.unwrap().parse::<u32>().unwrap();
		total_fuel += fuel_for_mass(mass);
	}

	println!("Total fuel needed is {}", total_fuel);

	Ok(())
}

fn fuel_for_mass(mass: u32) -> u32 {
	if mass < 9 {
		0
	} else {
		let fuel =  mass / 3 - 2;
		fuel + fuel_for_mass(fuel)
	}
}