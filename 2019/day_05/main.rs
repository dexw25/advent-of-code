// Iteration on part 2, add IO and addressing modes

use std::fs::File;
use std::io::Read;
mod intcode_comp;
use intcode_comp::IntcodeComp;

fn main() -> std::io::Result<()> {
	// Open file
	let mut file = File::open("./TEST.txt")?;
	let mut buf = String::new();
	file.read_to_string(&mut buf)?;

	let ops = buf.split(","); // split on comma

	let mut mem_space: Vec<i64>= Vec::new(); // Could use with_capacity here for a speed optimization
	for i in ops {
		// Parse as int and push all found values into mem_space, ignore not-ints with a warning(split gives us one white space at the end so one is expected)
		match i.parse::<i64>() {
			Ok(num) => mem_space.push(num),
			Err(e) => println!("Warning: {}, string: {}", e, i),
		}
	}
	
	// Part 1
	let mut comp = IntcodeComp::new(&mem_space);
	comp.input(1);
	comp.run_all();
	let mut o: Vec<i64> = Vec::new();
	loop {
		match comp.output() {
			Some(val) => o.push(val),
			None => break,
		};
	}
	println!("Diagnostic Output, input 1: {:?}", o);

	// Part 2
	comp = IntcodeComp::new(&mem_space);
	comp.input(5);
	comp.run_all();
	o.clear();
	loop {
		match comp.output() {
			Some(val) => o.push(val),
			None => break,
		};
	}
	println!("Diagnostic Output, input 5: {:?}", o);

	Ok(())
}
