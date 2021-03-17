use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut buttons_iter = ('1'..='9').chain('A'..='D');
    let mut buttons = vec![vec!['\0'; 7]; 7];
    for (y, i) in (0..=2).chain((0..2).rev()).enumerate() {
        for j in 0..1 + 2 * i {
            buttons[y + 1][3 - i + j] = buttons_iter.next().unwrap();
        }
    }

    input.lines().fold((1, 3), |pos, line| {
        let (x, y) = line.chars().fold(pos, |(x, y), letter| {
            let (new_x, new_y) = match letter {
                'U' => (x, y - 1),
                'D' => (x, y + 1),
                'L' => (x - 1, y),
                'R' => (x + 1, y),
                _ => (x, y),
            };

            if buttons[new_y][new_x] == '\0' {
                (x, y)
            } else {
                (new_x, new_y)
            }
        });
        print!("{}", buttons[y][x]);
        (x, y)
    });
    println!("");
}
