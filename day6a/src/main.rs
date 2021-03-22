use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut counts = Vec::new();

    for line in input.lines() {
        for (n, letter) in line.chars().enumerate() {
            if n >= counts.len() {
                counts.push(std::collections::HashMap::new());
            }

            *counts[n].entry(letter).or_insert(0) += 1;
        }
    }

    println!("{}", counts.into_iter().map(
        |pos_counts| pos_counts.into_iter().max_by_key(|&(_, count)| count).unwrap().0
    ).collect::<String>());
}
