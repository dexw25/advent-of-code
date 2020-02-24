use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::fmt;
use std::convert::TryInto;

// Helper function to break down slope to minimal factors for tracing rays on a grid
fn minimize_slope (x_slope: i32, y_slope: i32) -> (i32, i32) {
	// Handle simple edge cases first with early returns
	if x_slope == 0 && y_slope == 0 {
		(0, 0)
	}else if x_slope == 0 {
		// Horizontal line
		(0, if y_slope > 0 {1} else {-1})
	} else if y_slope == 0 {
		// Vertical line
		(if x_slope > 0 {1} else {-1}, 0)
	} else {
		// Most complicated case, find the greatest common factor of the two components of the vector
		let mut gcf:i32 = if x_slope > y_slope {y_slope.abs()} else {x_slope.abs()};

		// Loop until GCF is 1 (irreducible) or it evenly divides both args
		while x_slope % gcf != 0 || y_slope % gcf != 0 && gcf > 1{
			gcf -= 1;
			assert_ne!(gcf, 0);
		}

		(x_slope / gcf, y_slope / gcf)
	}
}

// Data for each asteroid, derive sort from angle, then occlusions.
#[derive(PartialOrd, PartialEq, Debug)]
struct Rock {
	// Vague polar representation, angle and the number of occlusions are enough to get the order these will be lasered in
	angle: f32,
	occlusions: u32, // Number of rocks that occlude this rock
	coord: (i32, i32),
}

impl Rock {
	// Calculate angle and occlusions from origin and field
	fn new(origin: &(i32, i32), field: &Field, coord: &(i32, i32)) -> Rock {
		// Vector to coord from origin for the purposes of calculating angles
		let x: f32 = (coord.0 - origin.0) as f32; //
		let y: f32 = (coord.1 - origin.1) as f32;

		// Calculate the angle of the origin vector
		let mut angle = y.atan2(x); 

		// Apply some convenience transformations
		// 12:00 starts at -pi/2
		if angle >= -(std::f32::consts::FRAC_PI_2) {
			// Rotate points above this range by pi/2
			angle += std::f32::consts::FRAC_PI_2;
		} else {
			// Points that would be negative after the above transformation range [-pi:-pi/2], translate to [3pi/2:2pi]
			angle += std::f32::consts::PI * 2.0 + std::f32::consts::FRAC_PI_2;
		}

		// derive occlusion count (Could use angle information calculated above here, but this is simpler given the solution already existed)
		let occ = field.may_occlude(origin, coord);
		let diff: HashSet<_> = field.rocks.intersection(&occ).collect();
		let count = diff.len();

		Rock {
			angle: angle,
			occlusions: count as u32,
			coord: *coord
		}
	}
}


// Asteroid field representation, a hash set of all of the asteroid coordinates
struct Field {
	rocks: HashSet<(i32, i32)>,
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

	// add to a set of points occluded by 'point' when looking from 'origin' (set is &mut to optimize calling function slightly since this is called a lot)
	fn occluded_by(&self, origin: &(i32, i32), point: &(i32, i32), o_set: &mut HashSet<(i32, i32)>){
		let (x_slope, y_slope) = minimize_slope(point.0 - origin.0, point.1 - origin.1);
		
		// Initial condition, walk out from point
		let mut x = point.0;
		let mut y = point.1;

		// Trace the path, break when falling off the map
		loop {
			x += x_slope;
			y += y_slope;
			if x > self.x_max || x < 0 || y > self.y_max || y < 0 {break};
			o_set.insert((x, y));
		}
	}

	// Return a set of points that may occlude the point 
	fn may_occlude(&self, origin: &(i32, i32), point: &(i32, i32)) -> HashSet<(i32, i32)> {
		let mut ret: HashSet<(i32, i32)> = HashSet::new();
		// Like above, except we reverse the order since we want to trace the ray back to the origin
		let (x_slope, y_slope) = minimize_slope(origin.0 - point.0, origin.1 - point.1);
		
		// Initial condition, walk in from point
		let mut x = point.0;
		let mut y = point.1;

		// Trace the path, break when reached origin (do not push origin)
		loop {
			x += x_slope;
			y += y_slope;
			if x == origin.0 && y == origin.1 {break};
			ret.insert((x, y));
		}
		ret
	}

	// Return a list of the order of rocks that will be lasered
	fn laser_order(&self, origin: &(i32, i32), max_len: usize) -> Vec<(i32, i32)> {
		// Result
		let mut ret: Vec<(i32, i32)> = Vec::with_capacity(self.rocks.len()-1); // All rocks will be removed less the one we are perched on (@origin)\
		let mut temp: Vec<(Rock)> = Vec::with_capacity(self.rocks.len()-1);

		// Build list of rocks sorted by angle then by occlusions
		for r in self.rocks.iter() {
			// Skip over origin point
			if r != origin {
				temp.push(Rock::new(origin, self, &r));
			}
		}

		// Keep a running set of rocks we have destroyed
		let mut destroyed: HashSet<(i32, i32)> = HashSet::new();

		// Sort rocks
		let temp_len = temp.len();
		let mut temp_slice = temp.as_mut_slice();
		temp_slice.sort_by(|a, b| a.partial_cmp(b).unwrap());

		// Iterate through temp and push into ret until there are none left (or max length is satisfied)
		while temp_len > ret.len() && ret.len() < max_len {
			// Iterate over all rocks, pushing unoccluded rocks into ret, and assuming that on each pass through an occluded rock has one of its occlusions blasted
			for val in temp_slice.iter() {
				if val.occlusions == 0 {
					if destroyed.insert(val.coord) {
						ret.push(val.coord);
					}
				}
			}
			// Update occlusions (assuming all occlusions get decremented on each pass around)
			for i in 0..temp_slice.len() {
				if temp_slice[i].occlusions > 0 {
					temp_slice[i].occlusions -= 1;
				}
			}
		}
		ret
	}
}

impl fmt::Display for Field {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for y in 0..=self.y_max {
			for x in 0..=self.x_max {
				write!(f, "{}", match self.rocks.contains(&(x, y)) {
					true => "#",
					false => ".",// should be visually distinct enough
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

	// For the best rock, get the 200th asteroid to be lasered
	let order = f.laser_order(&best_rock, 200);
	println!("200th rock to be lasered is {:?}", order[199]);

	Ok(())
}