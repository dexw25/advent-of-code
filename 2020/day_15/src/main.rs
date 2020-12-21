use std::collections::HashMap;

// Play the number game
fn numbergame(start: &[usize], turns: usize) -> usize {
    let mut records: HashMap<usize, usize> = HashMap::new();

    // Initialize starting data
    for (i, num) in start.iter().enumerate() {
        records.insert(*num, i);
    }
    // Drop last number, so that next loop starts clean
    records.remove(start.last().unwrap());

    let mut last_number = *start.last().unwrap();
    for i in start.len()..turns {
        let next_number;
        // if last number was spoken before, say turns since last number
        if records.contains_key(&last_number) {
            next_number = (i - 1) - records.get(&last_number).unwrap(); // difference between last turn and the turn the number was spoken in before
        } else {
            // Else say 0
            next_number = 0;
        }
        records.insert(last_number, i - 1); // save number of last spoken number and the turn it was spoken in
        last_number = next_number; // reset for next run
    }

    last_number
}

fn main() {
    println!("{}", numbergame(&[0, 3, 1, 6, 7, 5], 2020));
    println!("{}", numbergame(&[0, 3, 1, 6, 7, 5], 30000000));
}

#[test]
fn basic1() {
    assert_eq!(numbergame(&[1, 3, 2], 2020), 1);
}

#[test]
fn basic2() {
    assert_eq!(numbergame(&[2, 1, 3], 2020), 10);
}

#[test]
fn basic3() {
    assert_eq!(numbergame(&[1, 2, 3], 2020), 27);
}

#[test]
fn basic4() {
    assert_eq!(numbergame(&[2, 3, 1], 2020), 78);
}

#[test]
fn basic5() {
    assert_eq!(numbergame(&[3, 2, 1], 2020), 438);
}

#[test]
fn basic6() {
    assert_eq!(numbergame(&[3, 1, 2], 2020), 1836);
}

#[test]
fn long_1() {
    assert_eq!(numbergame(&[0, 3, 6], 30000000), 175594);
}
#[test]
fn long_2() {
    assert_eq!(numbergame(&[1, 3, 2], 30000000), 2578);
}
#[test]
fn long_3() {
    assert_eq!(numbergame(&[2, 1, 3], 30000000), 3544142);
}
#[test]
fn long_4() {
    assert_eq!(numbergame(&[1, 2, 3], 30000000), 261214);
}
#[test]
fn long_5() {
    assert_eq!(numbergame(&[2, 3, 1], 30000000), 6895259);
}
#[test]
fn long_6() {
    assert_eq!(numbergame(&[3, 2, 1], 30000000), 18);
}
#[test]
fn long_7() {
    assert_eq!(numbergame(&[3, 1, 2], 30000000), 362);
}
