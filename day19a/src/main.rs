fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let count = input.parse().unwrap();
    let mut elves: Vec<_> = (1u32..count).chain([0].iter().copied()).collect();

    let mut pos = 0;
    while elves[pos] as usize != pos {
        elves[pos] = elves[elves[pos] as usize];
        pos = elves[pos] as usize;
    }
    println!("{}", pos + 1);
}
