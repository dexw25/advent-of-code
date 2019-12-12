use std::fs::File;
use std::io::Read;
use std::convert::TryInto;


// Run a pipeline of 5 intcode computers and find the inputs that maximise their outputs
fn maximise_thrusters(prog: &Vec<i32>) -> i32 {
	let run_pipeline = |phases: Vec<i32>| -> i32 {
		let mut output = 0;
		for i in 0..=4 {
			output = rocket_computer(prog, &vec![phases[i], output])[0];
		}
		output
	};
	// valid set of inputs is 0...=4
	let mut best_output:i32 = 0;

	// Each phase setting is used exactly once
	// there's definitely a more idomatic way to do this I might come back and clean this up when I discover it
	for a in 0..=4 {
		for b in 0..=4 {
			if b != a {for c in 0..=4 {
					if c != b && c != a {for d in 0..=4 {
							if d != a && d != b && d != c {for e in 0..=4 {
								if e != a && e != b && e != c && e != d {
									let out = run_pipeline(vec![a, b, c, d, e]);
									best_output = if out > best_output {out} else {best_output};
						}
					}}
				}}
			}}
		}
	}
	best_output

}

fn main() -> std::io::Result<()> {
	// Open file
	let mut file = File::open("./ACS.txt")?;
	let mut buf = String::new();
	file.read_to_string(&mut buf)?;

	let ops = buf.split(","); // split on comma

	let mut prog: Vec<i32>= Vec::new(); // Could use with_capacity here for a speed optimization
	for i in ops {
		// Parse as int and push all found values into mem_space, ignore not-ints with a warning(split gives us one white space at the end so one is expected)
		match i.parse::<i32>() {
			Ok(num) => prog.push(num),
			Err(e) => println!("Warning: {}, string: {}", e, i),
		}
	}

	// Test Cases
	assert_eq!(maximise_thrusters(&vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]), 43210);
	assert_eq!(maximise_thrusters(&vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]), 54321);
	assert_eq!(maximise_thrusters(&vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]), 65210);

	println!("Max signal: {}", maximise_thrusters(&prog));

	Ok(())
}

// Implementation of the computer generalized, for convenience mem_space is now copied to an internal mutable vector
fn rocket_computer(prog: &Vec<i32>, input: &Vec<i32>) -> Vec<i32>{
	let mut mem_space:Vec<i32> = prog.clone();

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