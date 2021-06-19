use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut empty = (0, 0);
    let mut leftmost_wall = u8::MAX;
    let mut w = 0;
    let mut h = 0;

    for line in input.lines().skip(2) {
        let mut words = line.split_whitespace();
        let mut coords = words.next().unwrap().split('-');
        let x = coords.nth(1).unwrap().trim_matches('x').parse::<u8>().unwrap();
        let y = coords.next().unwrap().trim_matches('y').parse::<u8>().unwrap();
        let size = words.next().unwrap().trim_matches('T').parse::<u16>().unwrap();
        let used = words.next().unwrap().trim_matches('T').parse::<u16>().unwrap();

        if x >= w {
            w = x + 1;
        }
        if y >= h {
            h = y + 1;
        }

        if used == 0 {
            empty = (x, y);
        } else if size > 100 && x < leftmost_wall {
            leftmost_wall = x;
        }
    }

    println!("{}", (empty.0 - leftmost_wall + 1) + empty.1 + (w - leftmost_wall) + (w - 2) * 5);
}
