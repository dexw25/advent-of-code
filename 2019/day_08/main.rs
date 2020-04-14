use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // Input is a sequence of single digits, so
    let mut img: Vec<u8>;
    {
        // Open file
        let mut file = File::open("./image.txt")?;
        img = Vec::with_capacity(file.metadata().unwrap().len().try_into().unwrap()); // vector needs a slot for every byte of this file
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        for i in buf.chars() {
            // Parse as int and push all found values into mem_space, ignore not-ints with a warning(split gives us one white space at the end so one is expected)
            match i.to_digit(10) {
                Some(num) => img.push(num.try_into().unwrap()),
                None => panic!("{} is not int", i),
            }
        }
    }

    // Layer dimensions
    let x = 25;
    let y = 6;
    let area = x * y;

    assert_eq!(img.len() % area, 0);

    // For each layer, first find the layer that has the fewest 0's for part 1
    let mut zero_count = std::u32::MAX;
    let mut ones = 0;
    let mut twos = 0;

    // Part 2 is done alongside part 1 because the loop iteration is very similar
    let mut fin: Vec<u8> = vec![2; area]; // Final image, rendered by collapsing all of the following layers down
    for layer in img.chunks(area) {
        // for layer in layers
        let mut counts: Vec<u32> = vec![0; 3];
        for (index, p) in layer.iter().enumerate() {
            // for pixel in layer
            counts[*p as usize] += 1; // update counts
                                      // If pixel is not already transparent, and p is not transparent commit p
            if fin[index] == 2 && *p != 2 {
                fin[index] = *p;
            }
        }

        // Update counts if necessary
        if zero_count > counts[0] {
            zero_count = counts[0];
            ones = counts[1];
            twos = counts[2];
        }
    }

    // checksum from part 1
    println!("Part 1: product of 1s and 2s is {}", ones * twos);

    // Render final image
    println!("Part 2: final image");
    for row in fin.chunks(x) {
        for col in row {
            print!("{}", if *col == 1 { "." } else { " " });
        }
        println!("");
    }

    Ok(())
}
