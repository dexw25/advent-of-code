// Orbit tree problem, store orbit relations in a hash map
use std::collections::HashMap;
use std::collections::HashSet;
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
		// just like find_path below but all this scope cares about is distance to COM, not the actual nodes in the path
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

// Input has no loops, find the closest parent common to A and B and the shortest path is the sum of each node's path to that parent
fn min_path(a_node: &str, b_node: &str, tree: &HashMap<String, String>) -> usize {
	// closure to generate set of nodes in path between a given leaf and COM
	let find_path = |a: &str| -> HashSet<&String> {
		let mut path = HashSet::new();
		let mut node = a.to_string();
		loop {
			match tree.get(&node) {
				Some(val) => {
					path.insert(val);
					if *val == "COM" {
						break
					} else {
						node = val.to_string();
					}
				},
				None => panic!("Key {} not related to COM", a),
			}
		}
		path
	};

	// Full paths cached, leaf to root
	let a_set = find_path(a_node);
	let b_set = find_path(b_node);

	let nodes_in_path: HashSet<_> = a_set.symmetric_difference(&b_set).collect();
	nodes_in_path.len() // all nodes in this set constitute the path, counting them provides the number of branches to traverse
}

#[test]
fn links_count() {
	assert_eq!(count_links(&init_tree("./test0.txt")), 42);
}
#[test]
fn sample_path() {
	assert_eq!(min_path("YOU", "SAN", &init_tree("./test1.txt")), 4);
}

fn main() -> std::io::Result<()> {
	// Do the work
	let map = init_tree("./input.txt");
	println!("Part 1 answer: {}", count_links(&map));

	// Part 2
	println!("Part 2 answer: {}", min_path("YOU", "SAN", &map));

	Ok(())
}