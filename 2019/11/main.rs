mod intcode_comp;
use intcode_comp::IntcodeComp;
use std::fs;
use std::collections::HashMap;
// use std::convert::TryInto;

#[derive(Copy, Clone)]
enum Color {
	Black=0,
	White=1
}

// Convert from i64 to colors
impl From<i64> for Color {
	fn from(val: i64) -> Self {
		match val {
			0 => Color::Black,
			1 => Color::White,
			_ => panic!("{} is not a valid color", val)
		}
	}
}

// State of the ship
struct Ship {
	def_color: Color, // Default color
	tiles: HashMap<(i32, i32), Color>,// hash map, maps coord tuple to colors
}

impl Ship {
	fn new(def: Color) -> Ship {
		Ship {
			def_color: def,
			tiles: HashMap::new()
		}
	}

	// Part 1 solution
	fn tiles_painted(&self) -> usize {
		self.tiles.len()
	}

	// get current color, return painted value if it exists else default
	fn get_color(&self, coord: &(i32, i32)) -> Color {
		match self.tiles.get(coord) {
			Some(c) => *c,
			None => self.def_color
		}
	}

	// Update tile with color
	fn set_color(&mut self, coord: (i32, i32), color: Color) {
		self.tiles.insert(coord, color);
	}
}

// Define directions a bot might be facing, count in clockwise seuence to make turns easy
#[derive(Clone, Copy)]
enum Direction {
	North=0,
	East=1,
	South=2,
	West=3,
}

enum Turn {
	Left=0,
	Right=1
}

// Convert from i64 to a turn
impl From<i64> for Turn {
	fn from(val: i64) -> Self {
		match val {
			0 => Turn::Left,
			1 => Turn::Right,
			_ => panic!("{} is not a valid turn", val)
		}
	}
}

// Define moves based on Turn
impl Direction {
	fn turn(self, dir: Turn) -> Self{
		use Direction::*;
		use Turn::*;
		match self {
			North => match dir {Left=>West, Right=>East},
			East => match dir {Left=>North, Right=>South},
			South => match dir {Left=>East, Right=>West},
			West => match dir {Left=>South, Right=>North},
		}
	}

}
// State of painting robot
struct Bot<'a> {
	facing: Direction,
	coord: (i32, i32),
	cpu: IntcodeComp,
	ship: &'a mut Ship, // ref to ship we are painting
}

impl Bot<'_> {
	// Initial state of bot, Ship has same lifetime as returned object
	fn new<'a>(s: &'a mut Ship, prog: &Vec<i64>) -> Bot<'a> {
		Bot {
			coord: (0, 0),
			cpu: IntcodeComp::new(prog),
			facing: Direction::North,
			ship: s
		}
	}

	fn step_forward(&mut self) {
		let (x, y) = self.coord;

		use Direction::*;
		self.coord = match self.facing {
			North => (x, y+1),
			East => (x+1, y),
			South => (x, y-1),
			West => (x-1, y)
		}
	}

	// Paint until robot halts, no return, just mutate state of ship
	fn paint(&mut self) {
		// Flow is always input first, then CPU outputs the color to paint the current tile and the direction to move
		self.cpu.input(self.ship.get_color(&self.coord) as i64);
		while self.cpu.eval_async() == true {
			// unwrap() here, if CPU pauses for input and does not provide 2 outputs this is a logic error
			self.ship.set_color(self.coord, Color::from(self.cpu.output().unwrap()));
			self.facing = self.facing.turn(Turn::from(self.cpu.output().unwrap()));

			// Take one step forward 
			self.step_forward();

			// Read current tile
			self.cpu.input(self.ship.get_color(&self.coord) as i64);
		}
		
		// TODO: Copy code from above and maybe make that bit a method
		match self.cpu.output() {
			Some(v) => self.ship.set_color(self.coord, Color::from(v)),
			None => ()
		};
	}
}

fn main() -> std::io::Result<()>{
	let buf = &fs::read("./painter.txt")?;
	// Convert to string, Trim whitespace, Split on commas, Parse as i64, Collect as vec
	let prog: Vec<i64> = std::str::from_utf8(buf).unwrap().trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect();

	let mut s = Ship::new(Color::Black);
	// Limit scope so that ships borrow is released and we can inspect it's state after the robot is done
	{
		let mut b = Bot::new(&mut s, &prog);
		b.paint();
	}

	println!("{} tiles were painted", s.tiles_painted());

	Ok(())
}