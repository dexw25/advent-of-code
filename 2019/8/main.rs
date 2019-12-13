use std::fs::File;
use std::io::Read;
use std::convert::TryInto;

fn main() -> std::io::Result<()> {
	// Open file
	let mut file = File::open("./image.txt")?;
	let mut buf = String::new();
	file.read_to_string(&mut buf)?;

	// Input is a sequence of single digits, so 
	let mut img: Vec<u8>= Vec::with_capacity(file.metadata().unwrap().len().try_into().unwrap()); // vector needs a slot for every byte of this file
	for i in buf.chars() {
		// Parse as int and push all found values into mem_space, ignore not-ints with a warning(split gives us one white space at the end so one is expected)
		match i.to_digit(10) {
			Some(num) => img.push(num.try_into().unwrap()),
			None => panic!("{} is not int", i),
		}
	}

	// Layer dimensions
	let x = 25;
	let y = 6;
	let area = x*y;
	assert_eq!(img.len() % area, 0);

	// For each layer, first find the layer that has the fewest 0's
	let mut zero_count = std::u32::MAX;
	let mut ones = 0;
	let mut twos = 0;
	for layer in img.chunks(area) { // for layer in layers
		let mut counts: Vec<u32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
		for p in layer { // for pixel in layer
			counts[*p as usize] += 1;
		}
		if zero_count > counts[0] {
			zero_count = counts[0];
			ones = counts[1];
			twos = counts[2];
		}
	}

	println!("Product of 1s and 2s is {}", ones * twos);

	Ok(())
}

