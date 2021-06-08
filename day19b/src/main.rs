fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let count = input.parse().unwrap();
    let mut elves: Vec<_> = (1u32..count).chain([0].iter().copied()).collect();

    let mut pos = count as usize / 2 - 1;
    let mut odd = count % 2 == 1;
    while elves[pos] as usize != pos {
        elves[pos] = elves[elves[pos] as usize];
        if odd {
            pos = elves[pos] as usize;
        }
        odd = !odd;
    }
    println!("{}", pos + 1);
}
