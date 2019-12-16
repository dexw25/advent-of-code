use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::fmt;
use std::convert::TryInto;

// Asteroid field representation, a hash set of all of the asteroid coordinates
struct Field {
	rocks: HashSet<(u32, u32)>, // (x, y)
}

impl Field {
	fn new (input: &String) -> Field {
		let mut map: HashSet<(u32, u32)> = HashSet::new();

		// Parse the input grid row by row, a # is a rock, a . is nothing 
		for (row, l) in input.lines().enumerate() {
			for (col, c) in l.chars().enumerate() {
				match c {
					'#' => map.insert((col.try_into().unwrap(), row.try_into().unwrap())),
					_ => true
				};
			}
		}
		Field {rocks: map}
	}
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// Find bounds for the grid
		let mut x_max = 0;
		let mut y_max = 0;
		for a in self.rocks.iter() {
			if x_max < a.0 {x_max = a.0;}
			if y_max < a.1 {y_max = a.1;}
		}
		for y in 0..=y_max {
			for x in 0..x_max {
				write!(f, "{}", match self.rocks.contains(&(x, y)) {
					true => "#",
					false => " ",
				})?;
			}
			writeln!(f, "")?;
		}
		Ok(())
    }
}

fn main() -> std::io::Result<()> {
	let f: Field;
	{
		// Open file
		let mut file = File::open("./input.txt")?;
		let mut buf = String::new();
		file.read_to_string(&mut buf)?;
		
		f = Field::new(&buf);
	}	

	println!("{}", f);

	Ok(())
}