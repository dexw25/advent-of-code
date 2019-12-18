use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::fmt;
use std::convert::TryInto;
// use std::iter::FromIterator;

// Asteroid field representation, a hash set of all of the asteroid coordinates
struct Field {
	rocks: HashSet<(i32, i32)>, // (x, y)
	x_max: i32,
	y_max: i32
}

impl Field {
	fn new (input: &String) -> Field {
		let mut x = 0;
		let mut y = 0;
		let mut map: HashSet<(i32, i32)> = HashSet::new();

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

	fn count_visible(&self, point: &(i32, i32)) -> usize {
		// Make sure the set contains the passed point
		assert_eq!(self.rocks.contains(&point), true);

		// Tally up all asteroids
		// Count the number of asteroids that are occluded using 'point' as the origin
		// the difference is the number visible
		let mut occluded_set: HashSet<(i32, i32)> = HashSet::new();
		occluded_set.insert(*point); // Do not count the point we care about

		for a in self.rocks.iter() {
			if a != point {
				self.occluded_by(&point, a, &mut occluded_set)
			}
		}

		let diff: HashSet<_> = self.rocks.difference(&occluded_set).collect();
		let ret: usize = diff.len();// # of asteroids remaining is the number that is visible, occluded set must be a subset of local_set

		ret
	}

	// Return a list of points occluded by 'point' when looking from 'origin'
	fn occluded_by(&self, origin: &(i32, i32), point: &(i32, i32), o_set: &mut HashSet<(i32, i32)>){
		// Rise and Run initial slope (must reduce to get set of all occluded points)
		let x_slope: i32 = point.0 - origin.0;
		let y_slope: i32 = point.1 - origin.1;
		let x_minslope: i32;
		let y_minslope: i32;

		// Handle simple edge cases first
		if x_slope == 0 {
			assert_ne!(y_slope, 0); // y_slope cannot be zero here, then we would be checking for occludes between a point and itself which has no meaning
			x_minslope = 0;
			y_minslope = if y_slope > 0 {1} else {-1};
		} else if y_slope == 0 {
			x_minslope = if x_slope > 0 {1} else {-1};
			y_minslope = 0;
		} else if x_slope.abs() == 1 {
			x_minslope = if x_slope > 0 {1} else {-1};
			y_minslope = y_slope;
		} else if y_slope.abs() == 1 {
			x_minslope = x_slope;
			y_minslope = if y_slope > 0 {1} else {-1};
		} else {
			// Most complicated case, find the greatest common factor of the two components of the vector
			let mut gcf:i32 = if x_slope > y_slope {y_slope.abs()} else {x_slope.abs()};

			// Search for greatest common factor, this is fast enough for small numbers
			while x_slope % gcf != 0 || y_slope % gcf != 0 {
				gcf -= 1;
				assert_ne!(gcf, 0);
			}

			x_minslope = x_slope / gcf;
			y_minslope = y_slope / gcf;
		}
		
		// Initial condition, walk out from point
		let mut x = point.0;
		let mut y = point.1;

		// Trace the path, break when falling off the map

		loop {
			x += x_minslope;
			y += y_minslope;
			if x > self.x_max || x < 0 || y > self.y_max || y < 0 {break};
			o_set.insert((x, y));
		}
	}
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for y in 0..=self.y_max {
			for x in 0..=self.x_max {
				write!(f, "{}", match self.rocks.contains(&(x, y)) {
					true => "#",
					false => ".",// whitespace for not-asteroids
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

	println!("Input map: \n{}", f);

	// Find the point with the most points visible from it
	let mut best_rock: (i32, i32) = (0,0);
	let mut best_count: usize = 0;

	for r in f.rocks.iter() {
		let count = f.count_visible(r);
		if count > best_count {
			best_count = count;
			best_rock = *r;
		}
	}

	println!("Best count is {} at ({}, {})", best_count, best_rock.0, best_rock.1);

	Ok(())
}