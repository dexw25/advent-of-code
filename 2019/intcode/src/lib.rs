#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
    //TODO: Determine if it's worth unit testing anything in here
}
use std::convert::TryInto;
use std::collections::VecDeque;
use std::fs;

pub struct IntcodeComp {
    mem_space: Vec<i64>,
    program_counter: usize,
    in_buf: VecDeque<i64>,
    out_buf: VecDeque<i64>,
    rel_base: i64,
}

// Enum for addressing modes per spec
enum AddressMode {
    Positional, // 0
    Immediate, // 1
    Relative, // 2
}

// Instructions the intcode computer supports
enum Opcodes {
    Add,
    Multiply,
    Input,
    Output,
    Jnz,
    Jz,
    Comparelt,
    Compareq,
    Rbo,
    Halt,
}

impl Opcodes {
    fn from_usize(val: usize) -> Opcodes {
        match val {
            1 => Opcodes::Add,
            2 => Opcodes::Multiply,
            3 => Opcodes::Input,
            4 => Opcodes::Output,
            5 => Opcodes::Jnz,
            6 => Opcodes::Jz,
            7 => Opcodes::Comparelt,
            8 => Opcodes::Compareq,
            9 => Opcodes::Rbo,
            99 => Opcodes::Halt,
            _ => panic!("Opcode {} not valid", val)
        }
    }
}

// Closures etc could make this much much cleaner I might come back and clean it up later
impl IntcodeComp {
    pub fn new (prog: &Vec<i64>) -> IntcodeComp {
        let mem = prog.clone(); // Make a mutable clone of the program to work on in local memory
        let pc = 0;
        let i = VecDeque::new();
        let o = VecDeque::new();
        let rb = 0;
        IntcodeComp {
            mem_space: mem,
            program_counter: pc,
            in_buf: i,
            out_buf: o,
            rel_base: rb,
        }
    }

    pub fn _int_mem(&self) -> &Vec<i64>{
        &self.mem_space
    }

    pub fn input(&mut self, i: i64) {
        self.in_buf.push_back(i);
    }

    pub fn output(&mut self) -> Option<i64> {
        self.out_buf.pop_front()
    }

    // Return number of items in output queue
    pub fn output_available(&self) -> usize {
        self.out_buf.len()
    }

    // Convenience method to run until either a halt command or the core is starved of input
    pub fn run_all(&mut self) {
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

    // Memory access macros
    // Closures for common repeated operations (using closures to save on suuuper repetitive argument passing)
    fn addr_mode(dig: Option<&u8>) -> AddressMode {
        use self::AddressMode::*;
        match dig {
            Some(&2) => Relative,
            Some(&1) => Immediate,
            Some(&0)|None => Positional,
            Some(&e) => panic!("Imm bad: {}", e),
        }
    }

    fn op_fetch(&mut self, mode: AddressMode, pc_off: usize) -> i64{
        use self::AddressMode::*;
        match mode {
            Positional => {
                let ptr:usize = self.mem_space[self.program_counter+pc_off].try_into().unwrap();
                if ptr >= self.mem_space.len() {
                    // Ptr would access memory not currently allocated, grow our memory space to fit the need initializing new entries to 0
                    self.mem_space.resize(ptr+1, 0);
                }
                self.mem_space[ptr]
            },
            Immediate => self.mem_space[self.program_counter+pc_off],
            Relative => {
                let ptr:usize = (self.mem_space[self.program_counter+pc_off]+ self.rel_base).try_into().unwrap();
                // For relative, add a relative base register to ptr
                if ptr >= self.mem_space.len() {
                    // Ptr would access memory not currently allocated, grow our memory space to fit the need initializing new entries to 0
                    self.mem_space.resize(ptr+1, 0);
                }
                self.mem_space[ptr]
            }
        }
    }

    // very similar to op_fetch except for data direction and immediate is not supported
    fn write_back(&mut self, mode: AddressMode, pc_off: usize, data: i64) {
        use self::AddressMode::*;
        match mode {
            Positional => {
                let ptr = self.mem_space[self.program_counter+pc_off] as usize;
                if ptr >= self.mem_space.len() {
                    // Ptr would access memory not currently allocated, grow our memory space to fit the need initializing new entries to 0
                    self.mem_space.resize(ptr+1, 0);
                }
                self.mem_space[ptr] = data;
            },
            Relative => {
                let ptr:usize = (self.mem_space[self.program_counter+pc_off]+ self.rel_base).try_into().unwrap();
                // For relative, add a relative base register to ptr
                if ptr >= self.mem_space.len() {
                    // Ptr would access memory not currently allocated, grow our memory space to fit the need initializing new entries to 0
                    self.mem_space.resize(ptr+1, 0);
                }
                self.mem_space[ptr] = data;
            },
            Immediate => panic!("Attempted to write with immediate mode, this is not allowed"),
        };
    }

    // Implementation of the computer generalized, call to evaluate until output, return is true if continued execution is a thing, or false if the program has halted
    pub fn eval_async(&mut self) -> bool{
        // Push least significant digit first, then rest into array of digits for decoding
        fn decompose(n: &i64, digits: &mut Vec<u8>) {
            digits.push((n % 10) as u8);
            if *n >= 10 {
                decompose(&(n/10), digits)
            }
        }

        loop {
            // Break opcode into vec of digits for decoding of modes
            let mut digits: Vec<u8> = Vec::with_capacity(5); // 2 for opcode, 3 for mode bits
            decompose(&self.mem_space[self.program_counter], &mut digits);

            // use iterator to pop without having to reverse
            let mut it = digits.iter();
            let mut opcode:usize = *it.next().unwrap() as usize; // first digit must always exist

            // Second digit may exist
            opcode += match it.next() {
                Some(&i) => (i * 10) as usize,
                None => 0,
            };

            // Unwrap next 3 if they exist to determine address modes for operands and result
            let l_imm = IntcodeComp::addr_mode(it.next());
            let r_imm = IntcodeComp::addr_mode(it.next());
            let dst_imm = IntcodeComp::addr_mode(it.next());

            // This is the state machine that executes directions, 3 stages for each math-ish instruction, IO is similar but omits one or more steps
            // -Fetch
            // -Operate
            // -Writeback
            use self::Opcodes::*;
            match Opcodes::from_usize(opcode) {
                Add => {
                    // Operand fetch
                    let l = self.op_fetch(l_imm, 1);
                    let r = self.op_fetch(r_imm, 2);

                    // Operate on local "registers"
                    let result:i64 = l + r;

                    // Writeback
                    self.write_back(dst_imm, 3, result);

                    // add consumes 4 ints
                    self.program_counter += 4;
                },
                Multiply => {
                    // Operand fetch
                    let l = self.op_fetch(l_imm, 1);
                    let r = self.op_fetch(r_imm, 2);

                    // Operate on local "registers"
                    let result:i64 = l * r;

                    // Writeback
                    self.write_back(dst_imm, 3, result);

                    // add consumes 4 ints
                    self.program_counter += 4;
                },
                Input => { // input
                    // Match here only returns if input is needed but not available, to allow calling function to give us more
                    match self.in_buf.pop_front() {
                        Some(val) => self.write_back(l_imm, 1, val),
                        None => return true,
                    };

                    // input consumes 2 ints
                    self.program_counter += 2;
                },
                Output => { // output
                    let val = self.op_fetch(l_imm, 1);
                    self.out_buf.push_back(val);

                    // output consumes 2 ints
                    self.program_counter += 2;
                    // return true; // comment out so that the CPU only pauses on input
                },
                Jnz => { // jump if true (if input operand is nonzero)
                    // Operand fetch, same as math instructions plus logic for jump
                    let cond = match self.op_fetch(l_imm, 1) {
                        0 => false,
                        _ => true // any nonzero value means jump
                    };
                    let j_addr = self.op_fetch(r_imm, 2);

                    // Perform jump or not
                    if cond {
                        // Do jump
                        self.program_counter = j_addr.try_into().unwrap();
                    } else {
                        // business as usual
                        self.program_counter += 3;
                    }
                },
                Jz => { // jump if not true (if input operand is zero)
                    // Operand fetch, same as math instructions plus logic for jump
                    // Operand fetch, same as math instructions plus logic for jump
                    let cond = match self.op_fetch(l_imm, 1) {
                        0 => true, // zero means jump
                        _ => false,
                    };
                    let j_addr = self.op_fetch(r_imm, 2);

                    // Perform jump or not
                    if cond {
                        // Do jump
                        self.program_counter = j_addr.try_into().unwrap();
                    } else {
                        // business as usual
                        self.program_counter += 3;
                    }
                },
                Comparelt => { // Less than, write 1 to destination if first op is less than second, else write 0
                    // Operand fetch
                    let l = self.op_fetch(l_imm, 1);
                    let r = self.op_fetch(r_imm, 2);

                    // Operate on local "registers"
                    let result:i64 = match l < r {
                        true => 1,
                        false => 0
                    };

                    // Writeback
                    self.write_back(dst_imm, 3, result);

                    // < consumes 4 ints
                    self.program_counter += 4;
                },
                Compareq => { // equals, write 1 to destination if first op == second, else write 0
                    // Operand fetch
                    // Operand fetch
                    let l = self.op_fetch(l_imm, 1);
                    let r = self.op_fetch(r_imm, 2);

                    // Operate on local "registers"
                    let result:i64 = match l == r {
                        true => 1,
                        false => 0
                    };

                    // Writeback
                    self.write_back(dst_imm, 3, result);

                    // == consumes 4 ints
                    self.program_counter += 4;
                },
                Rbo => { // Adjust the relative base offset by this ops only parameter
                    self.rel_base += self.op_fetch(l_imm, 1);
                    self.program_counter += 2;
                }
                Halt => break,
            }
        }
        false
    }
}

pub fn prog_from_file(path: &str) -> Vec<i64> {
    let buf = &fs::read(path).unwrap();
    std::str::from_utf8(buf).unwrap().trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect()
}