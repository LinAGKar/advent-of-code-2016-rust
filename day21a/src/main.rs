use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut password: VecDeque<_> = "abcdefgh".chars().collect();

    for line in input.lines() {
        let mut words = line.split_whitespace();
        match words.next().unwrap() {
            "swap" => {
                let (a, b) = if words.next().unwrap() == "position" {
                    let a = words.next().unwrap().parse().unwrap();
                    let b = words.nth(2).unwrap().parse().unwrap();
                    (a, b)
                } else {
                    let a = words.next().unwrap().chars().next().unwrap();
                    let b = words.nth(2).unwrap().chars().next().unwrap();
                    (
                        password.iter().position(|&x| x == a).unwrap(),
                        password.iter().position(|&x| x == b).unwrap(),
                    )
                };
                password.swap(a, b);
            }

            "rotate" => {
                let direction = words.next().unwrap();
                let (right, count) = if direction == "based" {
                    let letter = words.nth(4).unwrap().chars().next().unwrap();
                    let index = password.iter().position(|&x| x == letter).unwrap();
                    (true, index + if index >= 4 { 2 } else { 1 })
                } else {
                    let right = direction == "right";
                    (right, words.next().unwrap().parse().unwrap())
                };
                for _ in 0..count {
                    if right {
                        let val = password.pop_back().unwrap();
                        password.push_front(val);
                    } else {
                        let val = password.pop_front().unwrap();
                        password.push_back(val);
                    }
                }
            }

            "reverse" => {
                let start: usize = words.nth(1).unwrap().parse().unwrap();
                let end: usize = words.nth(1).unwrap().parse().unwrap();
                let count = end - start + 1;
                for i in 0..count / 2 {
                    password.swap(start + i, end - i);
                }
            }

            "move" => {
                let src: usize = words.nth(1).unwrap().parse().unwrap();
                let dest: usize = words.nth(2).unwrap().parse().unwrap();
                let val = password.remove(src).unwrap();
                password.insert(dest, val);
            }

            _ => {}
        }
    }

    println!("{}", password.into_iter().collect::<String>());
}
