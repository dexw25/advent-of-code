#[cfg(test)]
mod day1_test {
    #[test]
    fn part1() {
        let data = include_str!("test.txt");

        let ret = crate::part_one(
            &data
                .lines()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>(),
        );

        assert_eq!(ret, 7);
    }

    #[test]
    fn part2() {
        let data = include_str!("test.txt");

        let ret = crate::part_two_thisfunctionnameistoolongfortriggeringclippy(
            &data
                .lines()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>(),
        );

        assert_eq!(ret, 5);
    }
}

// Simply iterate the list and count the number of increased elements
fn part_one(seq: &[i32]) -> i32 {
    let mut last = seq[0];
    let mut ret = 0;
    for i in seq {
        if i > &last {
            ret += 1;
        }
        last = *i;
    }

    ret
}

fn part_two_thisfunctionnameistoolongfortriggeringclippy(seq: &[i32]) -> i32 {
    let mut last = seq[0] + seq[1] + seq[2];
    let mut ret = 0;

    for i in 3..seq.len() {
        let sum = seq[i] + seq[i - 1] + seq[i - 2];

        if sum > last {
            ret += 1;
        }

        last = sum;
    }

    ret
}

fn main() {
    let data = include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    println!("part 1: {}", part_one(&data));
    println!(
        "part 2: {}",
        part_two_thisfunctionnameistoolongfortriggeringclippy(&data)
    );
}
