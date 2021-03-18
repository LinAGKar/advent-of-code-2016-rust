use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input.lines().filter(|line| {
        let sides: Vec<u16> = line
            .split_whitespace()
            .map(|side| side.parse().unwrap())
            .collect();
        let total: u16 = sides.iter().sum();

        !(0..3).any(|i| sides[i] >= total - sides[i])
    }).count());
}
