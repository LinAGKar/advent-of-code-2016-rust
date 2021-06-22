use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut c = 0;
    let mut d = 0;

    for line in input.lines() {
        let words: Vec<_> = line.split_whitespace().collect();
        if let Ok(num) = words[1].parse() {
            if words[0] == "cpy" && words[2] == "c" {
                c = num;
            } else if words[0] == "jnz" && words[2] == "d" {
                d = num;
            }
        }
    }

    const EGGS: u32 = 12;
    println!("{}", (2..=EGGS).product::<u32>() + c * d);
}
