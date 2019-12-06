// input.txt contains a program that is designed for the following system
//  instruction words are 4 comma separated ints, first int is 1, 2, or 99, second and third are the operands, 4th is where the result is stored. 
//  operands and result are all pointers into the instruction stream, 1 and 2 add and multiply respectively, 99 signals end of program

use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
	// Open file
	let mut file = File::open("./input.txt")?;
	let mut buf = String::new();
	file.read_to_string(&mut buf)?;

	let ops = buf.split(",");

	let mut mem_space = Vec::new();
	for i in ops {
		// Parse as int and push all found values into mem_space, ignore not-ints with a warning(split gives us one white space at the end so this will be ignored)
		match i.parse::<u32>() {
			Ok(num) => mem_space.push(num),
			Err(e) => println!("Warning: {}", e),
		}
	}

	// Initial state modification required by problem description
	mem_space[1] = 12;
	mem_space[2] = 2; 

	// First some test cases
	let mut test_space1 = vec![1, 0, 0, 0, 99];
	let mut test_space2 = vec![2, 3, 0, 3, 99];
	let mut test_space3 = vec![2,4,4,5,99,0];
	let mut test_space4 = vec![1,1,1,4,99,5,6,0,99];
	rocket_computer(&mut test_space1);
	rocket_computer(&mut test_space2);
	rocket_computer(&mut test_space3);
	rocket_computer(&mut test_space4);
	assert_eq!(test_space1, [2, 0, 0, 0, 99]);
	assert_eq!(test_space2, [2, 3, 0, 6, 99]);
	assert_eq!(test_space3, [2,4,4,5,99,9801]);
	assert_eq!(test_space4, [30,1,1,4,2,5,6,0,99]);

	// Part 1
	rocket_computer(&mut mem_space);

	println!("Value at position 0 after part 1: {}", mem_space[0]);

	Ok(())
}

// Implementation of the computer generalized, mem_space is borrowed mutably
fn rocket_computer(mem_space: &mut Vec<u32>){
	let mut program_counter = 0;
	loop {
		let opcode = mem_space[program_counter];
		match opcode {
			1 => {
				let l = mem_space[program_counter+1] as usize;
				let r = mem_space[program_counter+2] as usize;
				let writeback = mem_space[program_counter+3] as usize;
				mem_space[writeback] = mem_space[l] + mem_space[r];
			},
			2 => {
				let l = mem_space[program_counter+1] as usize;
				let r = mem_space[program_counter+2] as usize;
				let writeback = mem_space[program_counter+3] as usize;
				mem_space[writeback] = mem_space[l] * mem_space[r];
			},
			99 => break,
			_ => println!("ERROR"),
		}
		program_counter += 4;
	}
}
