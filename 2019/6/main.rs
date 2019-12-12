// Orbit tree problem, store orbit relations in a hash map
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::string::String;

fn init_tree(path: &str) -> HashMap<String, String> {
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
		rels.insert(child.to_string(), parent.to_string());
	}
	rels
}

fn count_links(rels: &HashMap<String, String>) -> u32 {

	// Calculate total number of links, walk the tree and sum the number of steps for each leaf
	let mut result:u32 = 0;
	for k in rels.keys() {
		let mut node = k.clone();
		loop {
			match rels.get(&node) {
			Some(val) => {
				result += 1;
				if *val == "COM" {
					break
				} else {
					node = val.to_string();
				}
			}, 
			None => panic!("Key {} has no parent", k),
			}
		}
	}
	result
}

fn min_path(a_node: &str, b_node: &str, tree: &HashMap<String, String>) -> u32 {
	0
}

fn main() -> std::io::Result<()> {
	
	// test case
	{
		let map = init_tree("./test0.txt");
		assert_eq!(count_links(&map), 42);
	}
	// Do the work
	let map = init_tree("./input.txt");
	println!("Part 1 answer: {}", count_links(&map));

	// Part 2
	println!("Part 2 answer: {}", min_path("YOU", "SAN", &map));

	Ok(())
}