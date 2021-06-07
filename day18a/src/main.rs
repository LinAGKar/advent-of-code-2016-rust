const ROWS: usize = 40;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let w = input.chars().count();
    let mask = !(!0 << w);
    let row = input.chars().fold(0u128, |row, tile| {
        row << 1 | if tile == '^' { 1 } else { 0 }
    });

    println!("{}", (0..ROWS).fold((row, 0), |(row, count), _| {
        (
            (row << 1 ^ row >> 1) & mask,
            count + (0..w).filter(|x| row >> x & 0b1 == 0).count(),
        )
    }).1);
}
