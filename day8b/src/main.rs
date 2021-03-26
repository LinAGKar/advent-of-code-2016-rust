use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut display = vec![[false; 50].iter().copied().collect::<VecDeque<_>>(); 6];

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let first = words.next().unwrap();
        if first == "rect" {
            let mut coords = words.next().unwrap().split('x').map(|x| x.parse().unwrap());
            let w = coords.next().unwrap();
            let h = coords.next().unwrap();
            for x in 0..w {
                for y in 0..h {
                    display[y][x] = true;
                }
            }
        } else {
            let mode = words.next().unwrap();
            let pos: usize = words.next().unwrap().split('=').nth(1).unwrap().parse().unwrap();
            let steps = words.nth(1).unwrap().parse().unwrap();
            if mode == "row" {
                let row = &mut display[pos];
                for _ in 0..steps {
                    let item = row.pop_back().unwrap();
                    row.push_front(item);
                }
            } else {
                let mut col = [false; 6];
                for i in 0..col.len() {
                    col[i] = display[i][pos];
                }
                for i in 0..col.len() {
                    display[(i + steps) % col.len()][pos] = col[i];
                }
            }
        }
    }

    for line in display {
        for pix in line {
            print!("{}", if pix { '#' } else { ' ' });
        }
        println!("");
    }
}
