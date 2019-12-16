use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::fmt;
use std::convert::TryInto;
use std::iter::FromIterator;

// Asteroid field representation, a hash set of all of the asteroid coordinates
struct Field {
	rocks: HashSet<(u32, u32)>, // (x, y)
	x_max: u32,
	y_max: u32
}

impl Field {
	fn new (input: &String) -> Field {
		let mut x = 0;
		let mut y = 0;
		let mut map: HashSet<(u32, u32)> = HashSet::new();

		// Parse the input grid row by row, a # is a rock, a . is nothing 
		for (row, l) in input.lines().enumerate() {
			if y < row {y = row;}
			for (col, c) in l.chars().enumerate() {
				match c {
					'#' => map.insert((col.try_into().unwrap(), row.try_into().unwrap())),
					_ => true
				};
				// This doesn't need to run except for the first line, but will expand input to allow for non uniform line lengths
				if x < col {x = col;}
			}
		}
		Field {rocks: map, x_max: x.try_into().unwrap(), y_max: y.try_into().unwrap()}
	}

	fn count_visible(&self, point: (u32, u32)) -> u32 {
		// Make sure the set contains the passed point
		assert_eq!(self.rocks.contains(&point), true);

		// Copy the set minus point argument
		let mut local_set: HashSet<&(u32, u32)> = HashSet::from_iter(self.rocks.iter());
		local_set.remove(&point);

		// for pt in local_set: remove all points that are occluded by this point
		0
	}
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for y in 0..=self.y_max {
			for x in 0..self.x_max {
				write!(f, "{}", match self.rocks.contains(&(x, y)) {
					true => "#",
					false => " ",// whitespace for not-asteroids
				})?;
			}
			writeln!(f, "")?; // newlines in between each row
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