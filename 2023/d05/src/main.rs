use d05::Almanac;

#[allow(clippy::unwrap_used)]
fn main() -> Result<(), String> {
    let input = include_str!("input.txt");
    let test = include_str!("test.txt");

    let almanac = test.parse::<Almanac>().unwrap();

    dbg!(almanac.min_seed_loc()?);
    dbg!(almanac.min_seed_loc_v2()?);

    Ok(())
}
