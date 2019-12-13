use std::convert::TryInto;
use std::collections::VecDeque;

pub struct IntcodeComp {
	mem_space: Vec<i32>,
	program_counter: usize,
	in_buf: VecDeque<i32>, 
	out_buf: VecDeque<i32>
}

// Read only attributes to execute test cases on intcode core
struct IntcodeTest <'a>{
	program: &'a Vec<i32>,
	final_state: Option<Vec<i32>>,
	input: Option<Vec<i32>>,
	output: Option<Vec<i32>>,
}

// CLosures etc could make this much much cleaner I might come back and clean it up later 
impl IntcodeComp {
	fn new (prog: &Vec<i32>) -> IntcodeComp {
		let mem = prog.clone();
		let pc = 0;
		let i = VecDeque::new();
		let o = VecDeque::new();
		IntcodeComp {
			mem_space: mem,
			program_counter: pc,
			in_buf: i,
			out_buf: o,
		}
	}

	// Execute all of the intcode test cases that have been so far presented (may go unused)
	pub fn _self_test() -> std::io::Result<()> {
		// Statically define test sets
		let gt_prog = vec![3,9,8,9,10,9,4,9,99,-1,8];
		let zero_prog = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
		let tests = [
			IntcodeTest {
				program : &vec![1, 0, 0, 0, 99],
				final_state : Some(vec![2, 0, 0, 0, 99]),
				input : None,
				output : None},
			IntcodeTest {
				program : &vec![2, 3, 0, 3, 99],
				final_state : Some(vec![2, 3, 0, 6, 99]),
				input : None,
				output : None},
			IntcodeTest {
				program : &vec![2,4,4,5,99,0],
				final_state : Some(vec![2,4,4,5,99,9801]),
				input : None,
				output : None},
			IntcodeTest {
				program : &vec![1,1,1,4,99,5,6,0,99],
				final_state : Some(vec![30,1,1,4,2,5,6,0,99]),
				input : None,
				output : None}, 
			IntcodeTest {
				program: &gt_prog,
				final_state : None,
				input : Some(vec![1]),
				output : Some(vec![0])},
			IntcodeTest {
				program: &gt_prog,
				final_state : None,
				input : Some(vec![8]),
				output : Some(vec![1])},
			IntcodeTest {
				program: &zero_prog,
				final_state : None,
				input : Some(vec![0]),
				output : Some(vec![0])},
			IntcodeTest {
				program: &zero_prog,
				final_state : None,
				input : Some(vec![5]),
				output : Some(vec![1])},
			];
		for test in tests.iter() {
			let mut comp = IntcodeComp::new(test.program);

			// Apply inputs and run the core until done
			match &test.input {
				Some(vec) => for i in vec {comp.input(*i)},
				None => ()
			}
			comp.run_all();

			// Check outputs and final state if provided
			match &test.final_state {
				Some(vec) => if &comp._int_mem() != &vec {panic!("Final state check fail")},
				None => (),//pass
			};

			let mut test_out: Vec<i32> = vec![];
			loop {
				match comp.output() {
					Some(val) => test_out.push(val),
					None => break,
				}
			}

			match &test.output {
				Some(vec) => if &test_out != vec {panic!("Output sequence check fail")}
				None => () // Don't check
			}
		}
		Ok(())
	}

	fn _int_mem(&self) -> &Vec<i32>{
		&self.mem_space
	}

	fn input(&mut self, i: i32) {
		self.in_buf.push_back(i);
	}

	fn output(&mut self) -> Option<i32> {
		self.out_buf.pop_front()
	}

	// Convenience method to run until either a halt command or the core is starved of input
	fn run_all(&mut self) {
		// starvation must occur twice
		let mut starved = false;
		loop {
			match self.eval_async() {
				true => {
					if self.in_buf.len() == 0 {
						if starved {
							println!("intcode CPU input starved in run_all, this is probably not what you want");
							break;
						} else {
							starved = true;
						}
					} // else continue
				},
				false => {
					break; // Completed execution
				}
			}
		}
	}

	// Implementation of the computer generalized, call to evaluate until output, return is true if continued execution is a thing, or false if the program has halted
	fn eval_async(&mut self) -> bool{
		// Push least significant digit first, then rest into array of digits for decoding
		fn decompose(n: &i32, digits: &mut Vec<u8>) {
			digits.push((n % 10) as u8);
			if *n >= 10 {
				decompose(&(n/10), digits)
			}
		}

		// persistent state vars of Core
		let mut digits: Vec<u8> = vec![];
		loop {
			// Break opcode into digits for decoding of modes
			decompose(&self.mem_space[self.program_counter], &mut digits);

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
				Some(&e) => panic!("Imm bad: {} from [{}]=={}", e, self.program_counter, self.mem_space[self.program_counter]),
			};
			let r_imm = match it.next() {
				Some(&1) => true,
				Some(&0)|None => false,
				Some(&e) => panic!("Imm bad: {} from [{}]=={}", e, self.program_counter, self.mem_space[self.program_counter]),
			};
			let dst_imm = match it.next() {
				Some(&1) => true,
				Some(&0)|None => false,
				Some(&e) => panic!("Imm bad: {} from [{}]=={}", e, self.program_counter, self.mem_space[self.program_counter]),
			};

			// This is the state machine that executes directions, 3 stages for each math-ish instruction, IO is similar but omits one or more steps
			// -Fetch
			// -Operate
			// -Writeback
			match opcode {
				1 => {
					// Operand fetch
					let l = match l_imm {
						true => self.mem_space[self.program_counter+1] as i32,
						false => {
							let l_ptr = self.mem_space[self.program_counter+1] as usize;
							self.mem_space[l_ptr] as i32
						},
					};
					let r = match r_imm {
						true => self.mem_space[self.program_counter+2] as i32,
						false => {
							let r_ptr = self.mem_space[self.program_counter+2] as usize;
							self.mem_space[r_ptr] as i32
						},
					};

					// Operate on local "registers"
					let result:i32 = l + r;

					// Writeback
					match dst_imm {
						true => panic!("immedate mode not allowed on dst for opcode 1"),
						false => {
							let dst_ptr = self.mem_space[self.program_counter+3] as usize;
							self.mem_space[dst_ptr] = result;
						},
					};

					// add consumes 4 ints
					self.program_counter += 4;
				},
				2 => {
					// Operand fetch
					let l = match l_imm {
						true => self.mem_space[self.program_counter+1] as i32,
						false => {
							let l_ptr = self.mem_space[self.program_counter+1] as usize;
							self.mem_space[l_ptr] as i32
						},
					};
					let r = match r_imm {
						true => self.mem_space[self.program_counter+2] as i32,
						false => {
							let r_ptr = self.mem_space[self.program_counter+2] as usize;
							self.mem_space[r_ptr] as i32
						},
					};

					// Operate on local "registers"
					let result = l * r;

					// Writeback
					match dst_imm {
						true => panic!("immedate mode not allowed on dst for opcode 2"),
						false => {
							let dst_ptr = self.mem_space[self.program_counter+3] as usize;
							self.mem_space[dst_ptr] = result;
						},
					};

					// mul consumes 4 ints
					self.program_counter += 4;
				},
				3 => { // input
					// l_imm encodes the mode for single parameter instructions
					match l_imm {
						true => panic!("Immediate mode not allowed for input"),
						false => {
							let dst_ptr = self.mem_space[self.program_counter+1] as usize;
							self.mem_space[dst_ptr] = match self.in_buf.pop_front(){
								Some(val) => val,
								None => return true, // return and wait for input
							};
						}
					}

					// input consumes 2 ints
					self.program_counter += 2;
				},
				4 => { // output
					match l_imm {
						true => self.out_buf.push_back(self.mem_space[self.program_counter + 1]),
						false => {
							let output_ptr = self.mem_space[self.program_counter+1] as usize;
							self.out_buf.push_back(self.mem_space[output_ptr]);
						}
					}

					// output consumes 2 ints
					self.program_counter += 2;
					return true;
				},
				5 => { // jump if true (if input operand is nonzero)
					// Operand fetch, same as math instructions plus logic for jump
					let cond = match l_imm {
						true => if self.mem_space[self.program_counter+1] != 0 {true} else {false},
						false => {
							let l_ptr = self.mem_space[self.program_counter+1] as usize;
							if self.mem_space[l_ptr] != 0 {true} else {false}
						},
					};
					let j_addr = match r_imm {
						true => self.mem_space[self.program_counter+2] as i32,
						false => {
							let r_ptr = self.mem_space[self.program_counter+2] as usize;
							self.mem_space[r_ptr] as i32
						},
					};

					// Perform jump or not
					if cond {
						// Do jump
						self.program_counter = j_addr.try_into().unwrap();
					} else {
						// business as usual
						self.program_counter += 3;
					}
				},
				6 => { // jump if not true (if input operand is zero)
					// Operand fetch, same as math instructions plus logic for jump
					let cond = match l_imm {
						true => if self.mem_space[self.program_counter+1] == 0 {true} else {false},
						false => {
							let l_ptr = self.mem_space[self.program_counter+1] as usize;
							if self.mem_space[l_ptr] == 0 {true} else {false}
						},
					};
					let j_addr = match r_imm {
						true => self.mem_space[self.program_counter+2] as i32,
						false => {
							let r_ptr = self.mem_space[self.program_counter+2] as usize;
							self.mem_space[r_ptr] as i32
						},
					};

					// Perform jump or not
					if cond {
						// Do jump
						self.program_counter = j_addr.try_into().unwrap();
					} else {
						// business as usual
						self.program_counter += 3;
					}
				},
				7 => { // Less than, write 1 to destination if first op is less than second, else write 0
					// Operand fetch
					let l = match l_imm {
						true => self.mem_space[self.program_counter+1] as i32,
						false => {
							let l_ptr = self.mem_space[self.program_counter+1] as usize;
							self.mem_space[l_ptr] as i32
						},
					};
					let r = match r_imm {
						true => self.mem_space[self.program_counter+2] as i32,
						false => {
							let r_ptr = self.mem_space[self.program_counter+2] as usize;
							self.mem_space[r_ptr] as i32
						},
					};

					// Operate on local "registers"
					let result:i32 = if l < r {1}  else {0};

					// Writeback
					match dst_imm {
						true => panic!("immedate mode not allowed on dst for opcode 7"),
						false => {
							let dst_ptr = self.mem_space[self.program_counter+3] as usize;
							self.mem_space[dst_ptr] = result;
						},
					};

					// < consumes 4 ints
					self.program_counter += 4;
				},
				8 => { // equals, write 1 to destination if first op == second, else write 0
					// Operand fetch
					let l = match l_imm {
						true => self.mem_space[self.program_counter+1] as i32,
						false => {
							let l_ptr = self.mem_space[self.program_counter+1] as usize;
							self.mem_space[l_ptr] as i32
						},
					};
					let r = match r_imm {
						true => self.mem_space[self.program_counter+2] as i32,
						false => {
							let r_ptr = self.mem_space[self.program_counter+2] as usize;
							self.mem_space[r_ptr] as i32
						},
					};

					// Operate on local "registers"
					let result:i32 = if l == r {1}  else {0};

					// Writeback
					match dst_imm {
						true => panic!("immedate mode not allowed on dst for opcode 8"),
						false => {
							let dst_ptr = self.mem_space[self.program_counter+3] as usize;
							self.mem_space[dst_ptr] = result;
						},
					};

					// == consumes 4 ints
					self.program_counter += 4;
				},
				99 => break,
				_ => panic!("ERROR opcode {} not recognized", opcode),
			}

			// Clear opcode decoding information for next instruction
			digits.clear()
		}
		false
	}
}


fn main() -> std::io::Result<()> {
	IntcodeComp::_self_test()
}
