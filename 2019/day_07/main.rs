use std::fs::File;
use std::io::Read;
use intcode::IntcodeComp;

// Run a pipeline of 5 intcode computers and find the inputs that maximise their outputs
fn maximise_thrusters(prog: &Vec<i64>, min_phase: i64, max_phase: i64) -> i64 {
	let run_pipeline = |phases: Vec<i64>| -> i64 {
		let mut output = 0;
		let mut comps:Vec<IntcodeComp> = vec![];

		// Initialize 5 instances of intcode computers
		for i in 0..=4 {
			comps.push(IntcodeComp::new(prog));
			comps[i].input(phases[i]);
		}

		// Execute feedback loop until cores terminate
		let mut running = true;
		while running {
			// Run each computer in sequence
			for i in 0..=4 {
				comps[i].input(output);
				println!("Thread {} running", i);
				if comps[i].eval_async(){
					// Could just input(output()) here but we need to tap the output node for our uses
					output = match comps[i].output(){
						Some(val) => val,
						None => panic!("Output expected but none present")
					};
					println!("thread {} outputed {}", i, output);
				} else {
					println!("thread {} finished", i);
					// False return follows a true return w/last data, all cores return false at the same step through feedback
					running = false;
				}
			}
		}
		output
	};

	// valid set of inputs is 0...=4
	let mut best_output:i64 = 0;

	// Each phase setting is used exactly once
	// there's definitely a more idomatic way to do this I might come back and clean this up when I discover it
	for a in min_phase..=max_phase {
		for b in min_phase..=max_phase {
			if b != a {for c in min_phase..=max_phase {
					if c != b && c != a {for d in min_phase..=max_phase {
							if d != a && d != b && d != c {for e in min_phase..=max_phase {
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

#[test]
fn max_1() {
	assert_eq!(maximise_thrusters(&vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], 0, 4), 43210);
}
#[test]
fn max_2() {
	assert_eq!(maximise_thrusters(&vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], 0, 4), 54321);
}
#[test]
fn max_3() {
	assert_eq!(maximise_thrusters(&vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], 0, 4), 65210);
}
#[test]
fn max_1_fb() {
	assert_eq!(maximise_thrusters(&vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5], 5, 9), 139629729);
}
#[test]
fn max_2_fb() {
	assert_eq!(maximise_thrusters(&vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10], 5, 9), 18216);
}
//TODO: Fix whatever broke this
fn main() -> std::io::Result<()> {
	// Open file
	let mut file = File::open("./ACS.txt")?;
	let mut buf = String::new();
	file.read_to_string(&mut buf)?;

	let ops = buf.split(","); // split on comma

	let mut prog: Vec<i64>= Vec::new(); // Could use with_capacity here for a speed optimization
	for i in ops {
		// Parse as int and push all found values into mem_space, ignore not-ints with a warning(split gives us one white space at the end so one is expected)
		match i.parse::<i64>() {
			Ok(num) => prog.push(num),
			Err(e) => println!("Warning: {}, string: {}", e, i),
		}
	}

	println!("Max signal no feedback: {}", maximise_thrusters(&prog, 0, 4));
	println!("Max with feedback: {}", maximise_thrusters(&prog, 5, 9));

	Ok(())
}

