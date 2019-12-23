use std::fs;
use std::fmt;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coord {
	x: i64,
	y: i64,
	z: i64
}

impl Coord {
	fn sum(&self) -> i64 {
		self.x.abs() + self.y.abs() + self.z.abs()
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
	pos: Coord,
	vel: Coord,
}

impl Moon {
	pub fn new(pos: Coord) -> Self {
		Moon {
			pos,
			vel: Coord {x:0, y:0, z:0}
		}
	}
}

struct Sim {
	bodies: Vec<Moon>
}

enum Axes {
	X,
	Y,
	Z
}

impl Sim {
	// Pass initial state by value and move it into this struct
	fn new (initial_state: Vec<Moon>) -> Self {
		Sim {bodies : initial_state}

	}

	// Update gravity
	fn apply_gravity(&mut self){
		for i in 0..self.bodies.len() {
			// Copy out position coord for processing
			let pos = self.bodies[i].pos;

			// Still not great, but better than a sort. Search all moons and calculate modifier by comparison and addition
			let mut accel_x: i64 = 0;
			let mut accel_y: i64 = 0;
			let mut accel_z: i64 = 0;
			for j in self.bodies.iter() {
				// add one for every body above this one, sub for every body below, ignore bodies at equal position
				accel_x += match pos.x - j.pos.x {
					1..=std::i64::MAX => -1,
					std::i64::MIN..=-1 => 1,
					0 => 0
				};
				// add one for every body above this one, sub for every body below, ignore bodies at equal position
				accel_y += match pos.y - j.pos.y {
					1..=std::i64::MAX => -1,
					std::i64::MIN..=-1 => 1,
					0 => 0
				};
				// add one for every body above this one, sub for every body below, ignore bodies at equal position
				accel_z += match pos.z - j.pos.z {
					1..=std::i64::MAX => -1,
					std::i64::MIN..=-1 => 1,
					0 => 0
				};
			}

			self.bodies[i].vel.x += accel_x;
			self.bodies[i].vel.y += accel_y;
			self.bodies[i].vel.z += accel_z;
		}
	}

	fn apply_velocity(&mut self) {
		for m in self.bodies.as_mut_slice() {
			m.pos.x += m.vel.x;
			m.pos.y += m.vel.y;
			m.pos.z += m.vel.z;
		}
	}

	// Step steps times
	fn step(&mut self, steps: usize) {
		for _i in 0..steps {
			// Apply gravity, mutate velocity based on position
			self.apply_gravity();

			// Apply velocity, mutate position based on velocity
			self.apply_velocity();
		}
	}

	// Total energy of system
	fn total_energy(&self) -> i64 {
		// Total energy is the sum of the absolute values of all components in the data format
		let mut total:i64 = 0;
		self.bodies.iter().for_each(|x| total += x.pos.sum()*x.vel.sum());
		total
	}
}

// init from initial state in this form: '<x=14, y=2, z=8>', wierd things will happen if any members are missing, whitespace is ignored
impl From<&str> for Moon {
	fn from(val: &str) -> Self {
		// Trim braces and split into variables and data
		let mut s = val.trim_matches(|c| c == '<' || c == '>').split_terminator(|c| c == ',' || c == '=');
		let mut ret = Coord {x:0, y:0, z:0};
		loop {
			match s.next() {
				Some(axis) => match axis.trim() {
					"x" => ret.x = s.next().unwrap().parse::<i64>().unwrap(), 
					"y" => ret.y = s.next().unwrap().parse::<i64>().unwrap(), 
					"z" => ret.z = s.next().unwrap().parse::<i64>().unwrap(),
					&_ => panic!("Unexpected character {} found in initializer string", axis)
				},
				None => break
			}
		}
		Moon::new(ret)
	}
}

impl fmt::Display for Moon {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "<P: ({}, {}, {}), V: ({}, {}, {})>", self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z)
    }
}

fn main() -> std::io::Result<()> {
	let buf = &fs::read("./initstate.txt")?;

	let moons: Vec<Moon> = std::str::from_utf8(buf).unwrap().trim().lines().map(|s| Moon::from(s)).collect();

	// Initialize sim with state from file: For each line create a new moon object and make a vector of them all
	let mut sim = Sim::new(std::str::from_utf8(buf).unwrap().trim().lines().map(|s| Moon::from(s)).collect());

	sim.step(1000);

	// //Print total energy in system
	println!("Total energy after 1000 steps: {}", sim.total_energy());
	// println!("{} steps until loop", sim.loop_len());


	Ok(())
}