use intcode::IntcodeComp;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
	// Open file, parse to program
	let mut file = File::open("./BOOST.txt")?;
	let mut buf = String::new();
	file.read_to_string(&mut buf)?;

	let ops = buf.split(","); // split on comma

	let mut prog: Vec<i64>= Vec::new(); // Could use with_capacity here for a speed optimization
	for i in ops {
		// Parse as int and push all found values into mem_space, ignore not-ints with a warning(split gives us one white space at the end so one is expected)
		match i.parse::<i64>() {
			Ok(num) => prog.push(num),
			Err(e) => println!("Warning: {}, string: {}", e, i),
		}
	}

	let mut comp = IntcodeComp::new(&prog);

	// Per spec, input 1 to program and print outputs
	comp.input(1);
	comp.run_all();

	let mut comp_out: Vec<i64> = Vec::new();
	loop {
		match comp.output() {
			Some(val) => comp_out.push(val),
			None => break,
		};
	}

	println!("BOOST output: {:?}", comp_out);

	// Reinit for second program run
	comp = IntcodeComp::new(&prog);
	comp.input(2); // Select sensor boost mode
	comp.run_all();
	comp_out = Vec::new();
	loop {
		match comp.output() {
			Some(val) => comp_out.push(val),
			None => break,
		};
	}
	println!("Ceres coordinates: {:?}", comp_out);

	Ok(())
}