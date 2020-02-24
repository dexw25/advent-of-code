use intcode::IntcodeComp;
use std::fs;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone)]
enum Color {
	Black,
	White
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

// State of the ship, simply the default color and a map of the tiles that have been painted
struct Ship {
	def_color: Color, // Default color
	tiles: HashMap<(i32, i32), Color>,// hash map, maps coord tuple to colors
}

// Ship implementation provides getters and setters for individual tiles and a format method
impl Ship {
	fn new(def: Color, first_tile: Option<Color>) -> Ship {
		let mut t = HashMap::new();
		match first_tile {
			Some(c) => t.insert((0, 0), c),
			None => None
		};
		Ship {
			def_color: def,
			tiles: t
		}
	}

	// Part 1 solution
	fn tiles_painted(&self) -> usize {
		self.tiles.len()
	}

	// get color of a tile, return painted value if it exists else default
	fn get_color(&self, coord: &(i32, i32)) -> Color {
		match self.tiles.get(coord) {
			Some(c) => *c,
			None => self.def_color
		}
	}

	// paint a tile
	fn set_color(&mut self, coord: (i32, i32), color: Color) {
		self.tiles.insert(coord, color);
	}

	// Derive size of grid by searching points that have been painted
	fn dimensions(&self) -> (i32, i32, i32, i32) {
		let mut x_min = std::i32::MAX;
		let mut x_max = std::i32::MIN;
		let mut y_min = std::i32::MAX;
		let mut y_max = std::i32::MIN;
		// Simple brute force max search
		for k in self.tiles.keys() {
			if k.0 > x_max {x_max = k.0};
			if k.1 > y_max {y_max = k.1};
			if k.0 < x_min {x_min = k.0};
			if k.1 < y_min {y_min = k.1};
		}
		(x_min, x_max, y_min, y_max)
	}
}

impl fmt::Display for Ship {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// Define our "character set"
		let render = |x| match x {&Color::White=>"#", &Color::Black=>" "};
		// Derive dimensions of traversed space
		let (x_min, x_max, y_min, y_max) = self.dimensions();

		// Print Rows from top to bottom, columns left to right
		let mut i = y_max;
		while i >= y_min {
			for col in x_min..=x_max {
				write!(f, "{}", match self.tiles.get(&(col, i)) {Some(c)=> render(c), None=>render(&self.def_color)})?;
			}
			write!(f, "\n")?;
			i -= 1;
		}

		Ok(())
    }
}

// Define directions a bot might be facing and enum for turn directions
// Order here is important to map to IO from CPU 
#[derive(Clone, Copy)]
enum Direction {
	North,
	East,
	South,
	West,
}

enum Turn {
	Left,
	Right
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

// Define move behavior based on Turn value
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

// Painter bot class
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
		while self.cpu.eval_async() == true || self.cpu.output_available() > 0 {
			// unwrap() here, if CPU pauses for input and does not provide 2 outputs this is a logic error
			self.ship.set_color(self.coord, Color::from(self.cpu.output().unwrap()));
			self.facing = self.facing.turn(Turn::from(self.cpu.output().unwrap()));

			// Take one step forward 
			self.step_forward();

			// Read current tile
			self.cpu.input(self.ship.get_color(&self.coord) as i64);
		}
	}
}

fn main() -> std::io::Result<()>{
	let buf = &fs::read("./painter.txt")?;
	// Convert to string, Trim whitespace, Split on commas, Parse as i64, Collect as vec
	let prog: Vec<i64> = std::str::from_utf8(buf).unwrap().trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect();
	{
		let mut s = Ship::new(Color::Black, None);
		// Limit scope so that ships borrow is released and we can inspect it's state after the robot is done
		{
			let mut b = Bot::new(&mut s, &prog);
			b.paint();
		}

		println!("Ship painted as follows({} tiles):\n{}", s.tiles_painted(), s);
	}
	{
		let mut s = Ship::new(Color::Black, Some(Color::White));
		// Limit scope so that ships borrow is released and we can inspect it's state after the robot is done
		{
			let mut b = Bot::new(&mut s, &prog);
			b.paint();
		}

		println!("Ship painted as follows({} tiles):\n{}", s.tiles_painted(), s);
	}

	Ok(())
}