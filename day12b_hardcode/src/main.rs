use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let iterations: u32 = lines.nth(2).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();
    let extra: u32 = lines.nth(2).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();
    let c: u32 = lines.nth(10).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();
    let d: u32 = lines.next().unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();

    println!("{}", (0..iterations + extra).fold((1, 1), |(a, b), _| (a + b, a)).0 + c * d);
}
