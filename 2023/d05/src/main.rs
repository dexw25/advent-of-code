use d05::Almanac;

#[allow(clippy::unwrap_used)]
fn main() {
    let input = include_str!("input.txt");
    let test = include_str!("test.txt");

    let almanac = input.parse::<Almanac>().unwrap();

    dbg!(almanac.min_seed_loc());
}
