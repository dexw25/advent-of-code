#[must_use]
pub fn sum_calibrations(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let num: Vec<u32> = l
                .chars()
                .filter(char::is_ascii_digit)
                .filter_map(|c| c.to_digit(10))
                .collect();

            num[0] * 10 + num[num.len() - 1]
        })
        .sum()
}

#[must_use]
pub fn sum_calibrations_v2(input: &str) -> u32 {}

#[cfg(test)]
mod test {
    use super::sum_calibrations;

    #[test]
    fn sample1() {
        let input = include_str!("test.txt");
        assert_eq!(sum_calibrations(input), 142);
    }

    #[test]
    fn sample2() {
        assert_eq!(sum_calibrations_v2(include_str!("test2.txt")), 281);
    }
}
