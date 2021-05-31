use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let number: u16 = input.parse().unwrap();

    let mut queue = BinaryHeap::new();
    let mut visited = vec![vec![false; 53]; 53];
    let mut count = 0;
    visited[1][1] = true;
    queue.push((Reverse(0), 0, 1i8, 1i8));

    while let Some((_, steps, x, y)) = queue.pop() {
        if steps > 50 {
            break;
        }
        count += 1;

        for &(nx, ny) in &[
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
        ] {
            if nx < 0 || ny < 0 {
                continue;
            }

            if visited[nx as usize][ny as usize] {
                continue;
            }

            visited[nx as usize][ny as usize] = true;

            let (x, y) = (nx as u16, ny as u16);
            let num = x * x + 3 * x + 2 * x * y + y + y * y + number;
            if (0..16).filter(|i| num >> i & 0b1 != 0).count() % 2 == 1 {
                continue;
            }

            queue.push((Reverse(steps + 1), steps + 1, nx, ny));
        }
    }

    println!("{}", count);
}
