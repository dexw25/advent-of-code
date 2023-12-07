use d03::Schematic;

fn main() {
    let input = include_str!("input.txt");

    let sch = Schematic::new(input);
    let sch_i = sch.part_numbers_iter().unwrap();

    dbg!(sch_i.sum::<u64>());

    let sch_g = sch.gear_ratios_iter();

    dbg!(sch_g.sum::<u64>());
}
