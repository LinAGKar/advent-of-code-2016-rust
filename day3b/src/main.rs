use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let numbers: Vec<Vec<u16>> = input.lines().map(|line| {
        line.split_whitespace()
            .map(|side| side.parse().unwrap())
            .collect()
    }).collect();

    println!("{}", numbers.chunks(3).flat_map(|group| {
        (0..3).map(move |i| group.iter().map(|line| line[i]).collect())
    }).filter(|sides: &Vec<_>| {
        let total: u16 = sides.iter().sum();
        !(0..3).any(|i| sides[i] >= total - sides[i])
    }).count());
}
