// Orbit tree problem, store orbit relations in a hash map
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn count_links(path: &str) -> u32 {
	let mut buf: String;
	{
		let mut file = File::open(path).unwrap();
		buf = String::new();
		file.read_to_string(&mut buf).unwrap();
	}

	let lines = buf.lines();
	let mut rels = HashMap::new();

	// key is child, value is key of parent
	for l in lines {
		let mut rel = l.split(")");
		let parent = rel.next().unwrap();
		let child = rel.next().unwrap();
		rels.insert(child, parent);
	}

	// Calculate total number of links, walk the tree and sum the number of steps for each leaf
	let mut result:u32 = 0;
	for k in rels.keys() {
		let mut node = k.clone();
		loop {
			match rels.get(node) {
			Some(val) => {
				result += 1;
				if *val == "COM" {
					break
				} else {
					node = val;
				}
			}, 
			None => panic!("Key {} has no parent", k),
			}
		}
	}
	result
}

fn main() -> std::io::Result<()> {
	
	// test case
	assert_eq!(count_links("./test0.txt"), 42);

	// Do the work
	println!("Part 1 answer: {}", count_links("./input.txt"));

	Ok(())
}