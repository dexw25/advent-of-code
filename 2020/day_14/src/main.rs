use crate::data::DATA;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

mod data;
// Instruction datatypes
struct Mask {
    clr_mask: u64,
    set_mask: u64,
    float_mask: u64,
}
struct Mem {
    addr: u64,
    data: u64,
}
enum Instruction {
    Mask(Mask),
    Mem(Mem),
}

// Create mask from mask instruction line
impl FromStr for Mask {
    type Err = ParseIntError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut clr_mask = 0;
        let mut set_mask = 0;
        let mut float_mask = 0;
        let data_start = text.find(" = ").unwrap() + 3;
        for (i, c) in text[data_start..].chars().rev().enumerate() {
            match c {
                '0' => {
                    clr_mask |= 1 << i;
                }
                '1' => {
                    set_mask |= 1 << i;
                }
                'X' => {
                    float_mask |= 1 << i;
                }
                _ => unreachable!(),
            }
        }
        Ok(Mask {
            clr_mask,
            set_mask,
            float_mask,
        })
    }
}

// Parse mem instruction directive
impl FromStr for Mem {
    type Err = ParseIntError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        // Search for all of our delimiters
        let addr_start = text.find('[').unwrap() + 1;
        let addr_end = text.find(']').unwrap();
        let data_start = text.find(" = ").unwrap() + 3;

        let addr = text[addr_start..addr_end].parse::<u64>().unwrap();
        let data = text[data_start..].parse::<u64>().unwrap();

        Ok(Mem { addr, data })
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match &text[..4] {
            "mask" => Ok(Instruction::Mask(text[4..].parse::<Mask>()?)),
            "mem[" => Ok(Instruction::Mem(text[3..].parse::<Mem>()?)),
            _ => panic!(format!("Bad input text: {}", text)),
        }
    }
}

// Run a sequence of Instructions and return the memory state after it finishes
fn run_all(instructions: Vec<Instruction>) -> HashMap<u64, u64> {
    // Initialize machine state, hashmap for memory to keep it sparse
    let mut clr_mask = 0; // represent as 2 bitmasks to keep it light
    let mut set_mask = 0;
    let mut mem: HashMap<u64, u64> = HashMap::new();

    use Instruction::*;
    for i in instructions.iter() {
        match i {
            Mask(op) => {
                clr_mask = op.clr_mask;
                set_mask = op.set_mask;
            }
            Mem(op) => {
                // Calculate data and update memory state
                let data = (op.data | set_mask) & !clr_mask; // apply masks
                mem.insert(op.addr, data);
            }
        }
    }

    mem
}

// Run with part 2 logic, modify not the data but the memory address instead
fn run_all_v2(instructions: Vec<Instruction>) -> HashMap<u64, u64> {
    // Initialize machine state, hashmap for memory to keep it sparse
    let mut set_mask = 0; // represent as 2 bitmasks to keep it light
    let mut float_mask = 0;
    let mut mem: HashMap<u64, u64> = HashMap::new();

    use Instruction::*;

    for i in instructions.iter() {
        match i {
            Mask(op) => {
                set_mask = op.set_mask;
                float_mask = op.float_mask;
            }
            Mem(op) => {
                // Calculate address base
                let addr_base = (op.addr | set_mask) & !(float_mask);
                let mut float_pos: Vec<u64> = Vec::with_capacity(36);
                let mut bit_count = 0;
                // Count bits in float_mask and track the positions of those bits
                for j in 0..=36 {
                    if (float_mask & (1 << j)) != 0 {
                        bit_count += 1;
                        float_pos.push(j);
                    }
                }

                // For all possible offsets (2^(float_pos.len()) - 1)
                for j in 0..2u64.pow(bit_count) {
                    let mut offset = 0; // accumulate calculated address

                    // Calculate the true offset by mapping bits of j to the address space
                    for (k, bit) in float_pos.iter().enumerate() {
                        offset += ((j & (1 << k)) >> k) << bit; // select bit, shift down to 0, shift up to position in bitmask
                    }
                    mem.insert(addr_base + offset, op.data);
                }
            }
        }
    }

    mem
}

fn main() -> Result<(), ()> {
    println!(
        "p1: {}",
        run_all(
            DATA.lines()
                .map(|line| line.parse::<Instruction>().unwrap())
                .collect()
        )
        .values()
        .sum::<u64>()
    );
    println!(
        "p2: {}",
        run_all_v2(
            DATA.lines()
                .map(|line| line.parse::<Instruction>().unwrap())
                .collect()
        )
        .values()
        .sum::<u64>()
    );

    Ok(())
}

#[test]
fn test_mask() {
    use Instruction::*;
    if let Mask(instr) = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        .parse::<Instruction>()
        .unwrap()
    {
        assert_eq!(instr.set_mask, 0x40);
        assert_eq!(instr.clr_mask, 0x2);
    } else {
        panic!();
    }
}

#[test]
fn test_mem() {
    use Instruction::*;
    if let Mem(instr) = "mem[8] = 42".parse::<Instruction>().unwrap() {
        assert_eq!(instr.addr, 8);
        assert_eq!(instr.data, 42);
    } else {
        panic!();
    }
}

#[test]
fn test_program1() {
    let prog = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    let sum: u64 = run_all(
        prog.lines()
            .map(|line| line.parse::<Instruction>().unwrap())
            .collect(),
    )
    .values()
    .sum();
    assert_eq!(sum, 165);
}

#[test]
fn test_program2() {
    let prog = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    // let mem = run_all_v2(
    //     prog.lines()
    //         .map(|line| line.parse::<Instruction>().unwrap())
    //         .collect(),
    // );
    // println!("{:#?}", mem);

    let sum: u64 = run_all_v2(
        prog.lines()
            .map(|line| line.parse::<Instruction>().unwrap())
            .collect(),
    )
    .values()
    .sum();
    assert_eq!(sum, 208);
}
