use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap};

const GOAL: (i8, i8) = (31, 39);

fn h((x, y): (i8, i8)) -> u8 {
    (GOAL.0 - x).abs() as u8 + (GOAL.1 - y).abs() as u8
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let number: u16 = input.parse().unwrap();

    let start = (1, 1);

    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let mut f_scores = HashMap::new();
    f_scores.insert(start, h(start));

    let mut open_set = BTreeSet::new();
    open_set.insert((f_scores[&start], Reverse(0), start));

    let mut occupied = HashMap::new();

    while let Some(&curr) = open_set.iter().next() {
        open_set.remove(&curr);
        let (_, _, pos) = curr;

        let g_score = g_scores[&pos];

        if pos == GOAL {
            println!("{}", g_score);
            break;
        }

        for &new_pos in &[
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ] {
            if new_pos.0 < 0 || new_pos.1 < 0 || *occupied.entry(new_pos).or_insert_with(|| {
                let (x, y) = new_pos;
                let (x, y) = (x as u16, y as u16);
                let num = x * x + 3 * x + 2 * x * y + y + y * y + number;
                (0..16).filter(|i| num >> i & 0b1 != 0).count() % 2 == 1
            }) {
                continue;
            }

            let tentative_g_score = g_score + 1;
            let old_g_score = g_scores.get(&new_pos).copied().unwrap_or(u8::MAX);

            if tentative_g_score < old_g_score {
                if let Some(&f_score) = f_scores.get(&new_pos) {
                    open_set.remove(&(f_score, Reverse(old_g_score), new_pos));
                }

                let f_score = tentative_g_score + h(new_pos);
                open_set.insert((f_score, Reverse(tentative_g_score), new_pos));
                f_scores.insert(new_pos, f_score);
                g_scores.insert(new_pos, tentative_g_score);
            }
        }
    }
}
