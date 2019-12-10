// Iteration on part 2, add IO and addressing modes

use std::fs::File;
use std::io::Read;
use std::convert::TryInto;


fn main() -> std::io::Result<()> {
	// Open file
	let mut file = File::open("./TEST.txt")?;
	let mut buf = String::new();
	file.read_to_string(&mut buf)?;

	let ops = buf.split(","); // split on comma

	let mut mem_space: Vec<i32>= Vec::new(); // Could use with_capacity here for a speed optimization
	for i in ops {
		// Parse as int and push all found values into mem_space, ignore not-ints with a warning(split gives us one white space at the end so one is expected)
		match i.parse::<i32>() {
			Ok(num) => mem_space.push(num),
			Err(e) => println!("Warning: {}, string: {}", e, i),
		}
	}
	// First some test cases to verify underlying functionality
	{
		let mut test_space1 = vec![1, 0, 0, 0, 99];
		let mut test_space2 = vec![2, 3, 0, 3, 99];
		let mut test_space3 = vec![2,4,4,5,99,0];
		let mut test_space4 = vec![1,1,1,4,99,5,6,0,99];
		rocket_computer(&mut test_space1, &vec![]); // Keep test cases with no input or output
		rocket_computer(&mut test_space2, &vec![]);
		rocket_computer(&mut test_space3, &vec![]);
		rocket_computer(&mut test_space4, &vec![]);
		assert_eq!(test_space1, [2, 0, 0, 0, 99]);
		assert_eq!(test_space2, [2, 3, 0, 6, 99]);
		assert_eq!(test_space3, [2,4,4,5,99,9801]);
		assert_eq!(test_space4, [30,1,1,4,2,5,6,0,99]);

		// Test cases for new opcodes, these programs check if input is == 8 or < 8
		assert_eq!(rocket_computer(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], &vec![1]), [0]);
		assert_eq!(rocket_computer(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], &vec![8]), [1]);
		assert_eq!(rocket_computer(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], &vec![1]), [1]);
		assert_eq!(rocket_computer(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], &vec![9]), [0]);
		assert_eq!(rocket_computer(&mut vec![3,3,1108,-1,8,3,4,3,99], &vec![1]), [0]);
		assert_eq!(rocket_computer(&mut vec![3,3,1108,-1,8,3,4,3,99], &vec![8]), [1]);
		assert_eq!(rocket_computer(&mut vec![3,3,1107,-1,8,3,4,3,99], &vec![1]), [1]);
		assert_eq!(rocket_computer(&mut vec![3,3,1107,-1,8,3,4,3,99], &vec![9]), [0]);

		// These return 0 if the input was 0 and 1 if it was non-zero
		assert_eq!(rocket_computer(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &vec![0]), [0]);
		assert_eq!(rocket_computer(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &vec![5]), [1]);
		assert_eq!(rocket_computer(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &vec![0]), [0]);
		assert_eq!(rocket_computer(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &vec![9]), [1]);

		// This longer example returns 999 if input is < 8, 1000 if == 8, 1001 if > 8
		assert_eq!(rocket_computer(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &vec![4]), [999]);
		assert_eq!(rocket_computer(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &vec![8]), [1000]);
		assert_eq!(rocket_computer(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &vec![10]), [1001]);
	}

	// Part 1

	println!("Diagnostic Output, input 1: {:?}", rocket_computer(&mut mem_space.clone(), &vec![1]));

	// Part 2
	println!("Diagnostic Output, input 5: {:?}", rocket_computer(&mut mem_space.clone(), &vec![5]));

	Ok(())
}



// Implementation of the computer generalized, mem_space is borrowed mutably
fn rocket_computer(mem_space: &mut Vec<i32>, input: &Vec<i32>) -> Vec<i32>{
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
	let mut output :Vec<i32> = vec![];
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
			Some(&e) => panic!("Imm bad: {} from [{}]=={}", e, program_counter, mem_space[program_counter]),
		};
		let r_imm = match it.next() {
			Some(&1) => true,
			Some(&0)|None => false,
			Some(&e) => panic!("Imm bad: {} from [{}]=={}", e, program_counter, mem_space[program_counter]),
		};
		let dst_imm = match it.next() {
			Some(&1) => true,
			Some(&0)|None => false,
			Some(&e) => panic!("Imm bad: {} from [{}]=={}", e, program_counter, mem_space[program_counter]),
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
			5 => { // jump if true (if input operand is nonzero)
				// Operand fetch, same as math instructions plus logic for jump
				let cond = match l_imm {
					true => if mem_space[program_counter+1] != 0 {true} else {false},
					false => {
						let l_ptr = mem_space[program_counter+1] as usize;
						if mem_space[l_ptr] != 0 {true} else {false}
					},
				};
				let j_addr = match r_imm {
					true => mem_space[program_counter+2] as i32,
					false => {
						let r_ptr = mem_space[program_counter+2] as usize;
						mem_space[r_ptr] as i32
					},
				};

				// Perform jump or not
				if cond {
					// Do jump
					program_counter = j_addr.try_into().unwrap();
				} else {
					// business as usual
					program_counter += 3;
				}
			},
			6 => { // jump if not true (if input operand is zero)
				// Operand fetch, same as math instructions plus logic for jump
				let cond = match l_imm {
					true => if mem_space[program_counter+1] == 0 {true} else {false},
					false => {
						let l_ptr = mem_space[program_counter+1] as usize;
						if mem_space[l_ptr] == 0 {true} else {false}
					},
				};
				let j_addr = match r_imm {
					true => mem_space[program_counter+2] as i32,
					false => {
						let r_ptr = mem_space[program_counter+2] as usize;
						mem_space[r_ptr] as i32
					},
				};

				// Perform jump or not
				if cond {
					// Do jump
					program_counter = j_addr.try_into().unwrap();
				} else {
					// business as usual
					program_counter += 3;
				}
			},
			7 => { // Less than, write 1 to destination if first op is less than second, else write 0
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
				let result:i32 = if l < r {1}  else {0};

				// Writeback
				match dst_imm {
					true => panic!("immedate mode not allowed on dst for opcode 7"),
					false => {
						let dst_ptr = mem_space[program_counter+3] as usize;
						mem_space[dst_ptr] = result;
					},
				};

				// < consumes 4 ints
				program_counter += 4;
			},
			8 => { // equals, write 1 to destination if first op == second, else write 0
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
				let result:i32 = if l == r {1}  else {0};

				// Writeback
				match dst_imm {
					true => panic!("immedate mode not allowed on dst for opcode 8"),
					false => {
						let dst_ptr = mem_space[program_counter+3] as usize;
						mem_space[dst_ptr] = result;
					},
				};

				// == consumes 4 ints
				program_counter += 4;
			},
			99 => break,
			_ => panic!("ERROR opcode {} not recognized", opcode),
		}

		// Clear opcode decoding information for next instruction
		digits.clear()
	}
	output
}
