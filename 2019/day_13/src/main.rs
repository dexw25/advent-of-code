use intcode::IntcodeComp;
use intcode::prog_from_file;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64
}

fn main() {
    let mut prog = prog_from_file("game.txt");
    let mut comp = IntcodeComp::new(&prog);
    comp.run_all();

    // Store screen state as hash map
    let mut screen: HashMap<Coord, i64> = HashMap::new();

    // output is in units of 3, X coord, Y coord, and id for what is at that tile
    while comp.output_available() > 3 {
        let x = comp.output().unwrap();
        let y = comp.output().unwrap();
        let id = comp.output().unwrap();
        let coord = Coord {x, y};

        screen.insert(coord, id);
    }

    // Total squares for part 1 check
    let mut squares = 0;
    for i in screen.values() {
        if *i == 2 {
            squares += 1;
        }
    }

    println!("Number of squares: {}", squares);

    // Set address 0 to 2 for free play, and reinitialize computer
    prog[0] = 2;
    comp = IntcodeComp::new(&prog);
    //TODO: Part 2. win the game and get the score
}
