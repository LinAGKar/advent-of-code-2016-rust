use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let buttons: Vec<Vec<u8>> = (0..3).map(|i| (i * 3 + 1..=i * 3 + 3).collect()).collect();

    input.lines().fold((1, 1), |pos, line| {
        let (x, y) = line.chars().fold(pos, |(x, y), letter| match letter {
            'U' => (x, if y > 0 { y - 1 } else { y }),
            'D' => (x, if y < 2 { y + 1 } else { y }),
            'L' => (if x > 0 { x - 1 } else { x }, y),
            'R' => (if x < 2 { x + 1 } else { x }, y),
            _ => (x, y),
        });
        print!("{}", buttons[y][x]);
        (x, y)
    });
    println!("");
}
