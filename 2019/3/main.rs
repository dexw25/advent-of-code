//Input is two long lines, each describes the path of a wire in the "fuel system"
// The first task is to find the closest crossing of the two wires to the origin (they share the origin and this does not count as a crossing)
// General approach is to build a big ol' list of all of the points in each wire and compare the lists to find crossings. 
// Distances are "manhattan distances" which is just a sum of abs(coords) assuming the origin is 0,0 (which I am deciding it is)

use std::fs::File;
use std::io::Read;
use std::fmt;

struct Point {
	x: i32,
	y: i32,
}

enum Vertical {
	X,
	Y,
}
	
struct Line {
	length: i32,
	dir: Vertical,
}

impl Point {
	// Distance to a specific point
	fn manhattan_dist(&self, other: &Point) -> i32 {
		((self.x - other.x).abs() + (self.y - other.y).abs())
	}
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

struct Wire {
	start: Point,
	segments: Vec<Line>,
}

impl Wire {
	fn new (path: &str) -> Wire {
		let steps = path.split(",");
		let mut list = Vec::new(); // TODO: replace with with_capacity for runtime optimization
		for step in steps {
			let mut buf = step.chars();
			list.push(match buf.next() {
				Some('R') => {
					Line {length: buf.as_str().parse::<u32>().unwrap() as i32, dir: Vertical::X}
				},
				Some('L') => {
					Line {length: -(buf.as_str().parse::<u32>().unwrap() as i32), dir: Vertical::X}
				},
				Some('U') => {
					Line {length: buf.as_str().parse::<u32>().unwrap() as i32, dir: Vertical::Y}
				},
				Some('D') => {
					Line {length: -(buf.as_str().parse::<u32>().unwrap() as i32), dir: Vertical::Y}
				},
				_ => {
					panic!("Not a valid move {}", buf.as_str())
				}
			});

		}

		// hardcode start to be the origin
		Wire {start: Point {x: 0, y: 0}, segments: list}
	}

	// Search for intersections in two wires, returns intersect points
	fn intersects(&self, other: &Wire) -> Vec<Point> {
		use crate::Vertical::{X, Y};
		let mut results: Vec<Point> = vec![];
		let mut self_point = Point {x: self.start.x, y: self.start.y};

		// Compare complete list of points in this wires path with complete list of points in others path
		for ln in self.segments.iter() {
			let mut other_point = Point {x: other.start.x, y: other.start.y};

			// Rather than check the direction of every self line and then other for each, just check the direction of other once
			match ln.dir {
				X => {
					for other_ln in other.segments.iter() {
						// Always update current point but only check for intersect if lines are orthogonal
						match other_ln.dir {
							X => other_point.x += other_ln.length,
							Y => {
								let x_range = self_point.x..(self_point.x + ln.length);
								let y_range = other_point.y..(other_point.y + other_ln.length);
								// IF other_ln is vertical, other_ln.x is in the range of x values covered by ln horizontal line, and ln horizontal line is in range of X values covered by us
								if x_range.contains(&other_point.x) && y_range.contains(&self_point.y) {
									results.push(Point {x: other_point.x, y: self_point.y}); // Simple geometry, use fixed X from vertical line and fixed Y from horizontal
								}
								other_point.y += other_ln.length;
							}
						}
					}
					self_point.x += ln.length;
				},
				Y => {
					for other_ln in other.segments.iter() {
						match other_ln.dir {
							Y => other_point.y += other_ln.length,
							X => {
								let x_range = other_point.x..(other_point.x + other_ln.length); 
								let y_range = self_point.y..(self_point.y + ln.length);
								if x_range.contains(&self_point.x) && y_range.contains(&other_point.y) {
									results.push(Point {x: self_point.x, y: other_point.y}); // See above
								}
								other_point.x += other_ln.length;
							},
						}

					}
					self_point.y += ln.length;
				}
			}
		}
		results
	}
}

fn main() -> std::io::Result<()> {
	let mut buf: String;
	{
		let mut file = File::open("./input.txt")?;
		buf = String::new();
		file.read_to_string(&mut buf)?;
	}

	let mut lines = buf.lines();

	// First two lines of the file are relevant, ignore the rest
	let wire1 = Wire::new(lines.next().unwrap());
	let wire2 = Wire::new(lines.next().unwrap());

	let mut shortest_distance = std::i32::MAX;
	for intersect in wire1.intersects(&wire2) {
		let dist_manhattan = intersect.manhattan_dist(&Point {x: 0, y: 0});
		println!("intersect: {}, distance to origin: {}", intersect, dist_manhattan);

		if dist_manhattan < shortest_distance {shortest_distance = dist_manhattan};
	}

	// Part 1
	println!("Shortest Distance by Manhattan measurement: {}", shortest_distance);

	Ok(())
}