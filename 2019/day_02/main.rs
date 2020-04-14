// input.txt contains a program that is designed for the following system
//  instruction words are 4 comma separated ints, first int is 1, 2, or 99, second and third are the operands, 4th is where the result is stored.
//  operands and result are all pointers into the instruction stream, 1 and 2 add and multiply respectively, 99 signals end of program

use intcode::IntcodeComp;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // Open file
    let mut file = File::open("./input.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let ops = buf.split(",");

    let mut mem_space = Vec::new(); // Could use with_capacity here for a speed optimization
    for i in ops {
        // Parse as int and push all found values into mem_space, ignore not-ints with a warning(split gives us one white space at the end so this will be ignored)
        match i.parse::<i64>() {
            Ok(num) => mem_space.push(num),
            Err(e) => println!("Warning: {}", e),
        }
    }

    // Part 1
    println!(
        "Verb: 12, Noun: 02 => {}",
        computer_result(&mem_space, 12, 2)
    );

    // Part 2, find verb and noun that return 19690720
    // Search space of verb and noun up to the highest value possible, IE the largest possible address. This is defined by the size of the program
    let prog_size = mem_space.len();
    let mut tries = 0;
    for verb in 0..prog_size {
        for noun in 0..prog_size {
            tries += 1;
            if computer_result(
                &mem_space,
                noun.try_into().unwrap(),
                verb.try_into().unwrap(),
            ) == 19690720
            {
                println!("Solved! noun={}, verb={}, {} tries", noun, verb, tries);
                return Ok(());
            }
        }
    }

    Ok(())
}

// Add in program/verb: noun abstraction and wrap the core below
fn computer_result(program: &Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut mem_space = program.to_vec(); // Make a mutable copy of the program to work in
    mem_space[1] = noun;
    mem_space[2] = verb;

    // Run the core until it terminates itself
    let mut comp = IntcodeComp::new(&mem_space);
    comp.run_all();

    // memory value 0 is the result
    comp._int_mem()[0]
}
