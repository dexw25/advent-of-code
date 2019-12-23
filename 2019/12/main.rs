use std::fs;
use std::fmt;


#[derive(Debug)]
struct Coord {
	x: i32,
	y: i32,
	z: i32
}

impl Coord {
	fn sum(&self) -> i32 {
		self.x.abs() + self.y.abs() + self.z.abs()
	}
}

#[derive(Debug)]
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

	// Apply velocity vector
	pub fn apply_velocity(&mut self) {
		self.pos.x += self.vel.x;
		self.pos.y += self.vel.y;
		self.pos.z += self.vel.z;
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

	// Closure to extract field from moon, passed as reference because we use it more than once in here
	fn g_update(&mut self, axis: Axes){
		// Save a closure to get the relevant axis based on arg, to keep following code concise
		let bodies = self.bodies.as_mut_slice();
		let pos_fun = |m: &Moon| match axis {
			Axes::X => m.pos.x,
			Axes::Y => m.pos.y,
			Axes::Z => m.pos.z,
		};

		for i in 0..bodies.len() {
			// Still not great, but better than a sort. Search all moons and calculate modifier by comparison and addition
			let coord = pos_fun(&bodies[i]);
			let mut accel: i32 = 0;
			for m in bodies.iter() {
				// add one for every body above this one, sub for every body below, ignore bodies at equal position
				accel += if pos_fun(m) > coord {1} else if pos_fun(m) < coord {-1} else {0};
			}

			match axis {
				Axes::Y => bodies[i].vel.y += accel,
				Axes::X => bodies[i].vel.x += accel,
				Axes::Z => bodies[i].vel.z += accel,
			};
		}
	}

	// Apply gravity transformation to velocity (Sorts bodies 3 times, if bodies is long this will be a long op)
	// Every moon on either side incurs a 1 unit change in velocity on the axis for which the position is different
	// If the coordinate is the same then the moon has no effect
	fn apply_gravity(&mut self) {		
		// X axis
		self.g_update(Axes::X);

		// Y axis
		self.g_update(Axes::Y);

		// Z axis
		self.g_update(Axes::Z);
	}

	fn apply_velocity(&mut self) {
		for m in self.bodies.as_mut_slice() {
			m.apply_velocity();
		}
	}

	// Step steps times
	fn step(&mut self, steps: u32) {
		for _i in 0..steps {
			// Apply gravity, mutate velocity based on position
			self.apply_gravity();

			// Apply velocity, mutate position based on velocity
			self.apply_velocity();
		}
	}

	// Total energy of system
	fn total_energy(&self) -> i32 {
		// Total energy is the sum of the absolute values of all components in the data format
		let mut total:i32 = 0;
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
					"x" => ret.x = s.next().unwrap().parse::<i32>().unwrap(), 
					"y" => ret.y = s.next().unwrap().parse::<i32>().unwrap(), 
					"z" => ret.z = s.next().unwrap().parse::<i32>().unwrap(),
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

	// For each line create a new moon object and make a vector of them all
	let moons: Vec<Moon> = std::str::from_utf8(buf).unwrap().trim().lines().map(|s| Moon::from(s)).collect();

	// Initialize sim with state from file
	let mut sim = Sim::new(moons);

	//DEBUG
	// sim.single_step();
	// Perform 1000 steps
	sim.step(1000);

	//Print total energy in system
	println!("Total energy after 1000 steps: {}", sim.total_energy());
	// println!("Final state: {:#?}", sim.bodies);

	Ok(())
}