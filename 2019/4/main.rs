// Calculate the number of valid passwords that fit given rules. 

// The rules are as follows: 
/*
    It is a six-digit number.
    The value is within the range given in your puzzle input.
    Two adjacent digits are the same (like 22 in 122345).
    Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
*/
// 

fn increasing(seq: &Vec<usize>) -> bool {
	let mut last:usize = 0;
	for i in seq.iter() {
		if last > *i {return false;}
		last = *i;
	}
	true
}

fn has_pair(seq: &Vec<usize>) -> bool {
	let mut last:usize = 10; // No digit in the input will be 10 or more
	for i in seq.iter() {
		if last == *i {return true;}
		last = *i;
	}
	false
}

fn main() -> std::io::Result<()> {
	let mut total:u32 = 0;
	let val_min = 367479;
	let val_max = 893698; // this is the puzzle input
	// 6 digit number enforced by loops
	for a in 0..10 {
		for b in 0..10 {
			for c in 0..10 {
				for d in 0..10 {
					for e in 0..10 {
						for f in 0..10 {
							// Needs to be checked both as a sequence and as an integer value so derive both
							let password = vec![a, b, c, d, e, f];
							let password_val = a * 100000 + b * 10000 + c * 1000 + d * 100 + e * 10 + f;
							// If the number isnt in the given range then do not do any further checks
							if password_val > val_min && password_val < val_max {
								if increasing(&password) && has_pair(&password) {
									total += 1;
								} 
							}
						}
					}
				}
			}
		}
	}

	println!("{}", total);

	Ok(())
}