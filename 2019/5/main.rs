// Iteration on part 2, add IO and addressing modes

use std::fs::File;
use std::io::Read;


fn main() -> std::io::Result<()> {
	// Open file
	let mut file = File::open("./TEST.txt")?;
	let mut buf = String::new();
	file.read_to_string(&mut buf)?;

	let ops = buf.split(","); // split on comma

	let mut mem_space: Vec<i32>= Vec::new(); // Could use with_capacity here for a speed optimization
	for i in ops {
		// Parse as int and push all found values into mem_space, ignore not-ints with a warning(split gives us one white space at the end so this will be ignored)
		match i.parse::<i32>() {
			Ok(num) => mem_space.push(num),
			Err(e) => println!("Warning: {}, string: {}", e, i),
		}
	}

	// First some test cases
	{
		let mut test_space1 = vec![1, 0, 0, 0, 99];
		let mut test_space2 = vec![2, 3, 0, 3, 99];
		let mut test_space3 = vec![2,4,4,5,99,0];
		let mut test_space4 = vec![1,1,1,4,99,5,6,0,99];
		rocket_computer(&mut test_space1, &vec![], &mut vec![]); // Keep test cases with no input or output
		rocket_computer(&mut test_space2, &vec![], &mut vec![]);
		rocket_computer(&mut test_space3, &vec![], &mut vec![]);
		rocket_computer(&mut test_space4, &vec![], &mut vec![]);
		assert_eq!(test_space1, [2, 0, 0, 0, 99]);
		assert_eq!(test_space2, [2, 3, 0, 6, 99]);
		assert_eq!(test_space3, [2,4,4,5,99,9801]);
		assert_eq!(test_space4, [30,1,1,4,2,5,6,0,99]);
	}

	let mut output: Vec<i32> = vec![];
	rocket_computer(&mut mem_space, &vec![1], &mut output);

	println!("Diagnostic Output: {:?}", output);

	Ok(())
}



// Implementation of the computer generalized, mem_space is borrowed mutably
fn rocket_computer(mem_space: &mut Vec<i32>, input: &Vec<i32>, output: &mut Vec<i32>){
	// Push least significant digit first, then rest into array of digits for decoding
	fn decompose(n: &i32, digits: &mut Vec<u8>) {
		digits.push((n % 10) as u8);
		if *n >= 10 {
			decompose(&(n/10), digits)
		}
	}

	// persistent state vars of Core
	let mut digits: Vec<u8> = vec![];
	let mut program_counter = 0;
	let mut input_idx = 0;
	loop {
		// Break opcode into digits for decoding of modes
		decompose(&mem_space[program_counter], &mut digits);

		// use iterator
		let mut it = digits.iter();
		let mut opcode:usize = *it.next().unwrap() as usize; // first digit must always exist

		// Second digit may exist
		opcode += match it.next() {
			Some(&i) => (i * 10) as usize,
			None => 0,
		};

		// Unwrap next 3 if they exist to determine address modes for operands and result
		let l_imm = match it.next() {
			Some(&1) => true,
			Some(&0)|None => false,
			_ => panic!(),
		};
		let r_imm = match it.next() {
			Some(&1) => true,
			Some(&0)|None => false,
			_ => panic!(),
		};
		let dst_imm = match it.next() {
			Some(&1) => true,
			Some(&0)|None => false,
			_ => panic!(),
		};

		// This is the state machine that executes directions, 3 stages for each math-ish instruction, IO is similar but omits one or more steps
		// -Fetch
		// -Operate
		// -Writeback
		match opcode {
			1 => {
				// Operand fetch
				let l = match l_imm {
					true => mem_space[program_counter+1] as i32,
					false => {
						let l_ptr = mem_space[program_counter+1] as usize;
						mem_space[l_ptr] as i32
					},
				};
				let r = match r_imm {
					true => mem_space[program_counter+2] as i32,
					false => {
						let r_ptr = mem_space[program_counter+2] as usize;
						mem_space[r_ptr] as i32
					},
				};

				// Operate on local "registers"
				let result:i32 = l + r;

				// Writeback
				match dst_imm {
					true => panic!("immedate mode not allowed on dst for opcode 1"),
					false => {
						let dst_ptr = mem_space[program_counter+3] as usize;
						mem_space[dst_ptr] = result;
					},
				};

				// add consumes 4 ints
				program_counter += 4;
			},
			2 => {
				// Operand fetch
				let l = match l_imm {
					true => mem_space[program_counter+1] as i32,
					false => {
						let l_ptr = mem_space[program_counter+1] as usize;
						mem_space[l_ptr] as i32
					},
				};
				let r = match r_imm {
					true => mem_space[program_counter+2] as i32,
					false => {
						let r_ptr = mem_space[program_counter+2] as usize;
						mem_space[r_ptr] as i32
					},
				};

				// Operate on local "registers"
				let result = l * r;

				// Writeback
				match dst_imm {
					true => panic!("immedate mode not allowed on dst for opcode 2"),
					false => {
						let dst_ptr = mem_space[program_counter+3] as usize;
						mem_space[dst_ptr] = result;
					},
				};

				// mul consumes 4 ints
				program_counter += 4;
			},
			3 => { // input
				// l_imm encodes the mode for single parameter instructions
				match l_imm {
					true => panic!("Immediate mode not allowed for input"),
					false => {
						let dst_ptr = mem_space[program_counter+1] as usize;
						mem_space[dst_ptr] = *input.get(input_idx).unwrap();
						input_idx += 1;
					}
				}

				// input consumes 2 ints
				program_counter += 2;
			},
			4 => { // output
				match l_imm {
					true => output.push(mem_space[program_counter + 1]),
					false => {
						let output_ptr = mem_space[program_counter+1] as usize;
						output.push(mem_space[output_ptr]);
					}
				}

				// output consumes 2 ints
				program_counter += 2;
			},
			99 => break,
			_ => panic!("ERROR opcode {} not recognized", opcode),
		}

		// Clear opcode decoding information for next instruction
		digits.clear()
	}
}
