use intcode::prog_from_file;
use intcode::IntcodeComp;

// Run a pipeline of 5 intcode computers and find the inputs that maximise their outputs
fn maximise_thrusters(prog: &[i64], min_phase: i64, max_phase: i64, feedback: bool) -> i64 {
    let run_pipeline = |phases: &[i64]| -> i64 {
        let mut output = 0;
        let mut comps: Vec<IntcodeComp> = Vec::with_capacity(phases.len());

        // Initialize an instance of intcode computer for each passed phase
        for &val in phases {
            let mut c = IntcodeComp::new(prog);
            c.input(val);
            comps.push(c);
        }

        // Execute feedback loop until cores terminate
        let mut running = true;
        while running {
            // Run each computer in sequence
            for comp in comps.iter_mut() {
                // Output contains last data from the last stage
                comp.input(output);

                // If comp terminated, signal that loop should end after this run
                if !comp.eval_async() {
                    running = false;
                }

                // Always take output if present
                output = match comp.output() {
                    Some(val) => val,
                    None => panic!("Output expected but none present"),
                };
            }
            if !feedback {
                break;
            }
        }
        output
    };

    let mut best_output: i64 = 0;

    // Each phase setting is used exactly once
    // there's definitely a more idomatic way to do this I might come back and clean this up when I discover it
    for a in min_phase..=max_phase {
        for b in min_phase..=max_phase {
            if b != a {
                for c in min_phase..=max_phase {
                    if c != b && c != a {
                        for d in min_phase..=max_phase {
                            if d != a && d != b && d != c {
                                for e in min_phase..=max_phase {
                                    if e != a && e != b && e != c && e != d {
                                        let out = run_pipeline(&[a, b, c, d, e]);
                                        best_output =
                                            if out > best_output { out } else { best_output };
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    best_output
}

#[test]
fn max_1() {
    assert_eq!(
        maximise_thrusters(
            &vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
            0,
            4,
            false
        ),
        43210
    );
}
#[test]
fn max_2() {
    assert_eq!(
        maximise_thrusters(
            &vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ],
            0,
            4,
            false
        ),
        54321
    );
}
#[test]
fn max_3() {
    assert_eq!(
        maximise_thrusters(
            &vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ],
            0,
            4,
            false
        ),
        65210
    );
}
#[test]
fn max_1_fb() {
    assert_eq!(
        maximise_thrusters(
            &vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ],
            5,
            9,
            true
        ),
        139629729
    );
}
#[test]
fn max_2_fb() {
    assert_eq!(
        maximise_thrusters(
            &vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ],
            5,
            9,
            true
        ),
        18216
    );
}

fn main() -> std::io::Result<()> {
    let prog = prog_from_file("./ACS.txt");

    println!(
        "Max signal no feedback: {}",
        maximise_thrusters(&prog, 0, 4, false)
    );
    println!(
        "Max with feedback: {}",
        maximise_thrusters(&prog, 5, 9, true)
    );

    Ok(())
}
