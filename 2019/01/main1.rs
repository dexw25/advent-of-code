// Read hardcoded path: input.txt and calculate the toal fuel need
// Each line is a moudle weight, implement the following algorithim:
//  "to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2"

use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
	let mut total_parts = 0;
	let mut total_mass = 0;
	let mut total_fuel = 0;

	// Open file
	let file = File::open("./input.txt")?;
	let file = BufReader::new(file);

	// iterate over lines and attempt to parse as int
	for line in file.lines() {
		let mass = line.unwrap().parse::<u32>().unwrap();
		total_parts +=1;
		total_mass += mass;
		if mass > 6 {
			total_fuel += mass / 3 - 2
		}
	}
	println!("got {} lines, total mass was {}", total_parts, total_mass);
	println!("Total fuel needed is {}", total_fuel);

	Ok(())
}
