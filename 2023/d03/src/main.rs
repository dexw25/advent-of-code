use d03::Schematic;

fn main() {
    let input = include_str!("input.txt");

    let sch = Schematic::new(input);
    let sch_i = sch.part_numbers_iter().unwrap();

    dbg!(sch_i.sum::<u32>());
}
