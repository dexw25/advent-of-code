use intcode::prog_from_file;
use intcode::IntcodeComp;

fn main() -> std::io::Result<()> {
    // Open file, parse to program
    let prog = prog_from_file("./BOOST.txt");
    let mut comp = IntcodeComp::new(&prog);

    // Per spec, input 1 to program and print outputs
    comp.input(1);
    comp.run_all();

    let mut comp_out: Vec<i64> = Vec::new();
    while let Some(val) = comp.output() {
        comp_out.push(val);
    }
    println!("BOOST output: {:?}", comp_out);

    // Reinit for second program run
    comp = IntcodeComp::new(&prog);
    comp.input(2); // Select sensor boost mode
    comp.run_all();
    comp_out = Vec::new();
    while let Some(val) = comp.output() {
        comp_out.push(val);
    }

    println!("Ceres coordinates: {:?}", comp_out);

    Ok(())
}
