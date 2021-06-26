use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut words = input.lines().map(|x| x.split_whitespace().nth(1).unwrap());
    let a: u16 = words.nth(1).unwrap().parse().unwrap();
    let b: u16 = words.next().unwrap().parse().unwrap();
    let min = a * b;

    println!("{}", (1..).map(|x| {
        !(!0 << 2 * x) & 0xAAAA
    }).find(|&x| x >= min).unwrap() - min);
}
