//Input is two long lines, each describes the path of a wire in the "fuel system"
// The first task is to find the closest crossing of the two wires to the origin (they share the origin and this does not count as a crossing)
// General approach is to build a big ol' list of all of the points in each wire and compare the lists to find crossings. 
// Distances are "manhattan distances" which is just a sum of abs(coords) assuming the origin is 0,0 (which I am deciding it is)

use std::fs::File;
use std::io::Read;
use std::convert::TryInto;
use std::fmt;

struct Point {
	x: i32,
	y: i32,
}

impl Point {
	// Distance to a specific point
	fn manhattan_dist(&self, other: &Point) -> u32 {
		((self.x - other.x).abs() + (self.y - other.y).abs()).try_into().unwrap()
	}
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

struct Wire {
	points: Vec<Point>,
}

impl Wire {
	fn new (path: &str) -> Wire {
		let mut list = Vec::new();
		let mut current_point = Point {x: 0, y: 0};
		let mut x_delta = 0;
		let mut y_delta = 0;
		let steps = path.split(",");
		for step in steps {
			let mut buf = step.chars();
			match buf.next() {
				Some('R') => x_delta = buf.as_str().parse::<u32>().unwrap() as i32,
				Some('L') => x_delta = -(buf.as_str().parse::<u32>().unwrap() as i32),
				Some('U') => y_delta = buf.as_str().parse::<u32>().unwrap() as i32,
				Some('D') => y_delta = -(buf.as_str().parse::<u32>().unwrap() as i32),
				_ => panic!("Not a valid move {}", buf.as_str())
			}
			if x_delta != 0 {
				if x_delta < 0 {
					while x_delta != 0 {
						x_delta += 1;
						current_point.x -= 1;
						list.push(Point {x: current_point.x, y: current_point.y});
					}
				} else if x_delta > 0 {
					while x_delta != 0 {
						x_delta -= 1;
						current_point.x += 1;
						list.push(Point {x: current_point.x, y: current_point.y});
					}
				}
			} else if y_delta != 0 {
				if y_delta < 0 {
					while y_delta != 0 {
						y_delta += 1;
						current_point.y -= 1;
						list.push(Point {x: current_point.x, y: current_point.y});
					}
				} else if y_delta > 0 {
					while y_delta != 0 {
						y_delta -= 1;
						current_point.y += 1;
						list.push(Point {x: current_point.x, y: current_point.y});
					}
				}
			} else {
				panic!("Not a valid move");
			}
		}
		Wire {points: list}
	}

	fn contains_point(&self, x: i32, y: i32) -> bool {
		for pt in self.points.iter() {
			if pt.x == x && pt.y == y {
				return true;
			}
		}
		false
	}

	// Suboptimal search for intersections in two wires, returns intersect points
	fn intersects(&self, other: &Wire) -> Vec<Point> {
		let mut results: Vec<Point> = vec![];

		// Compare complete list of points in this wires path with complete list of points in others path
		for pt in self.points.iter() {
			if other.contains_point(pt.x, pt.y) {
				results.push(Point {x: pt.x, y: pt.y});
			}
		}

		results
	}
}

fn main() -> std::io::Result<()> {
	let mut file = File::open("./input.txt")?;
	let mut buf = String::new();
	file.read_to_string(&mut buf)?;

	let mut lines = buf.lines();

	// First two lines of the file are relevant, ignore the rest
	let wire1 = Wire::new(lines.next().unwrap());
	let wire2 = Wire::new(lines.next().unwrap());

	for intersect in wire1.intersects(&wire2) {
		println!("intersect: {}, distance to origin: {}", intersect, intersect.manhattan_dist(&Point {x: 0, y: 0}));
	}

	Ok(())
}