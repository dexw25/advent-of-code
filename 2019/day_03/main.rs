//Input is two long lines, each describes the path of a wire in the "fuel system"
// The first task is to find the closest crossing of the two wires to the origin (they share the origin and this does not count as a crossing)
// General approach is to build a big ol' list of all of the points in each wire and compare the lists to find crossings.
// Distances are "manhattan distances" which is just a sum of abs(coords) assuming the origin is 0,0 (which I am deciding it is)

use std::fmt;
use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Vertical {
    X,
    Y,
}

#[derive(Debug)]
struct Line {
    length: i32,
    dir: Vertical,
}

impl Point {
    // Distance to a specific point
    fn manhattan_dist(&self, other: &Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
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
    fn new(path: &str) -> Wire {
        let steps = path.split(",");
        let mut list = Vec::new(); // could replace with with_capacity for runtime optimization but getting the length of the wire also has a cost at this point
        for step in steps {
            let mut buf = step.chars();
            list.push(match buf.next() {
                Some('R') => Line {
                    length: buf.as_str().parse::<u32>().unwrap() as i32,
                    dir: Vertical::X,
                },
                Some('L') => Line {
                    length: -(buf.as_str().parse::<u32>().unwrap() as i32),
                    dir: Vertical::X,
                },
                Some('U') => Line {
                    length: buf.as_str().parse::<u32>().unwrap() as i32,
                    dir: Vertical::Y,
                },
                Some('D') => Line {
                    length: -(buf.as_str().parse::<u32>().unwrap() as i32),
                    dir: Vertical::Y,
                },
                _ => panic!("Not a valid move {}", buf.as_str()),
            });
        }

        // hardcode start to be the origin
        Wire {
            start: Point { x: 0, y: 0 },
            segments: list,
        }
    }

    // Search for intersections in two wires, returns intersect points
    fn intersects(&self, other: &Wire) -> Vec<Point> {
        use crate::Vertical::{X, Y};
        let mut results: Vec<Point> = vec![];
        let mut self_point = Point {
            x: self.start.x,
            y: self.start.y,
        };

        // Compare complete list of points in this wires path with complete list of points in others path
        for ln in self.segments.iter() {
            let mut other_point = Point {
                x: other.start.x,
                y: other.start.y,
            };

            // Rather than check the direction of every self line and then other for each, just check the direction of other once
            match ln.dir {
                X => {
                    // Negative ranges don't work, swap order if start is less than end
                    let x_range = if ln.length > 0 {
                        self_point.x..=(self_point.x + ln.length)
                    } else {
                        (self_point.x + ln.length)..=self_point.x
                    };
                    for other_ln in other.segments.iter() {
                        // Always update current point but only check for intersect if lines are orthogonal
                        match other_ln.dir {
                            X => other_point.x += other_ln.length,
                            Y => {
                                let y_range = if other_ln.length > 0 {
                                    other_point.y..=(other_point.y + other_ln.length)
                                } else {
                                    (other_point.y + other_ln.length)..=other_point.y
                                };
                                // IF other_ln is vertical, other_ln.x is in the range of x values covered by ln horizontal line, and ln horizontal line is in range of X values covered by us
                                if x_range.contains(&other_point.x)
                                    && y_range.contains(&self_point.y)
                                {
                                    let p = Point {
                                        x: other_point.x,
                                        y: self_point.y,
                                    };
                                    if p.x != 0 && p.y != 0 {
                                        results.push(p);
                                    } // Simple geometry, use fixed X from vertical line and fixed Y from horizontal, ignore origin
                                }
                                other_point.y += other_ln.length;
                            }
                        }
                    }
                    self_point.x += ln.length;
                }
                Y => {
                    // Negative ranges don't work, swap order if start is less than end
                    let y_range = if ln.length > 0 {
                        self_point.y..=(self_point.y + ln.length)
                    } else {
                        (self_point.y + ln.length)..=self_point.y
                    };
                    for other_ln in other.segments.iter() {
                        match other_ln.dir {
                            Y => other_point.y += other_ln.length,
                            X => {
                                let x_range = if other_ln.length > 0 {
                                    other_point.x..=(other_point.x + other_ln.length)
                                } else {
                                    (other_point.x + other_ln.length)..=other_point.x
                                };
                                if x_range.contains(&self_point.x)
                                    && y_range.contains(&other_point.y)
                                {
                                    let p = Point {
                                        x: self_point.x,
                                        y: other_point.y,
                                    };
                                    if p.x != 0 && p.y != 0 {
                                        results.push(p);
                                    } // Simple geometry, use fixed X from vertical line and fixed Y from horizontal, ignore origin
                                }
                                other_point.x += other_ln.length;
                            }
                        }
                    }
                    self_point.y += ln.length;
                }
            }
        }
        results
    }

    // Calculate the length to the given point along the path of the wire, should probably check here if the point is on the wire
    fn path_to(&self, target: &Point) -> u32 {
        use crate::Vertical::{X, Y};
        let mut current_point = Point {
            x: self.start.x,
            y: self.start.y,
        }; // start at origin, not counted as a step
        let mut total_steps: u32 = 0;
        for ln in self.segments.iter() {
            match ln.dir {
                X => {
                    let mut x_left = ln.length;
                    while x_left != 0 {
                        if x_left > 0 {
                            current_point.x += 1;
                            x_left -= 1;
                        } else {
                            current_point.x -= 1;
                            x_left += 1;
                        }
                        total_steps += 1;
                        if current_point == *target {
                            return total_steps;
                        }
                    }
                }
                Y => {
                    let mut y_left = ln.length;
                    while y_left != 0 {
                        if y_left > 0 {
                            current_point.y += 1;
                            y_left -= 1;
                        } else {
                            current_point.y -= 1;
                            y_left += 1;
                        }
                        total_steps += 1;
                        if current_point == *target {
                            return total_steps;
                        }
                    }
                }
            }
        }
        total_steps
    }
}

// Feed input text files into the classes above and rank the output
fn find_crossings(path: &str) -> (u32, u32) {
    let mut buf: String;
    {
        let mut file = File::open(path).unwrap();
        buf = String::new();
        file.read_to_string(&mut buf).unwrap();
    }

    let mut lines = buf.lines();

    // First two lines of the file are relevant, ignore the rest
    let wire1 = Wire::new(lines.next().unwrap());
    let wire2 = Wire::new(lines.next().unwrap());

    let mut shortest_distance = std::u32::MAX;
    let mut shortest_path = std::u32::MAX;
    for intersect in wire1.intersects(&wire2) {
        let dist_manhattan = intersect.manhattan_dist(&wire1.start);
        let dist_path = wire1.path_to(&intersect) + wire2.path_to(&intersect); // path calculation from both not including the intersection itself, hence +1

        if dist_manhattan < shortest_distance {
            shortest_distance = dist_manhattan
        };
        if dist_path < shortest_path {
            shortest_path = dist_path
        };
    }

    (shortest_distance, shortest_path)
}

#[test]
fn test1() {
    assert_eq!(find_crossings("./test0.txt"), (6, 30));
}
#[test]
fn test2() {
    assert_eq!(find_crossings("./test1.txt"), (159, 610));
}
#[test]
fn test3() {
    assert_eq!(find_crossings("./test2.txt"), (135, 410));
}

fn main() -> std::io::Result<()> {
    let distances = find_crossings("./input.txt");

    println!("Part 1 solution: Manhattan Distance = {}", distances.0);
    println!("Part 2 solution: Path Distance = {}", distances.1);

    Ok(())
}
