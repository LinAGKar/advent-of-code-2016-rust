use std::cmp::Reverse;
use std::collections::{HashMap, BTreeSet};
use std::io::Read;

const ELEMS: u32 = 5;
const ITEMS: u32 = ELEMS * 2;

fn insert(state: u32, len: u32, val: u32) -> u32 {
    let pos = (0..len).find(|x| state >> x * 4 & 0xF <= val).unwrap_or(len);
    let mask = !0 << pos * 4;
    state & !mask | (state & mask) << 4 | val << (pos * 4)
}

fn get_neighbors(state: u32, neighbors: &mut Vec<u32>) {
    let elevator_pos = state >> (ITEMS * 2);
    let at_curr: Vec<_> = (0..ITEMS).filter(|&x| state >> x * 2 & 0b11 == elevator_pos).collect();
    for &i in [
        &[1u32] as &[u32],
        &[0u32, 2u32] as &[u32],
        &[1u32, 3u32] as &[u32],
        &[2u32] as &[u32],
    ][elevator_pos as usize] {
        let new_el_state = state & !(0b11 << ITEMS * 2) | i << ITEMS * 2;
        for &j in &at_curr {
            for &k in &at_curr {
                let new_state = (new_el_state & !(0b11 << j * 2) | i << j * 2)
                    & !(0b11 << k * 2) | i << k * 2;

                if !(0..ELEMS).all(|x| {
                    let chip_floor = new_state >> x * 4 + 2 & 0xF;
                    new_state >> x * 4 & 0b11 == chip_floor || (0..ELEMS).all(|y| new_state >> y * 4 & 0b11 != chip_floor)
                }) {
                    continue;
                }

                let pos_a = j >> 1;
                let pos_b = k >> 1;
                let a = new_state >> pos_a * 4 & 0xF;
                let b = new_state >> pos_b * 4 & 0xF;
                let mask_a = !0 << pos_a * 4;
                let new_state = new_state & !mask_a | (new_state & mask_a << 4) >> 4;
                let new_state = if pos_a == pos_b {
                    insert(new_state, ELEMS - 1, a)
                } else {
                    let pos_b = if pos_b > pos_a { pos_b - 1 } else { pos_b };
                    let mask_b = !0 << pos_b * 4;
                    let new_state = new_state & !mask_b | (new_state & mask_b << 4) >> 4;
                    let new_state = insert(new_state, ELEMS - 2, a);
                    insert(new_state, ELEMS - 1, b)
                };

                neighbors.push(new_state);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut elements = HashMap::new();

    for (floor, line) in input.lines().enumerate() {
        let mut generators = Vec::new();
        let mut chips = Vec::new();
        line.split_whitespace().fold("", |prev, word| {
            let word = word.trim_end_matches(&[',', '.'] as &[_]);
            if word == "generator" {
                generators.push(prev);
            } else if word == "microchip" {
                chips.push(prev.trim_end_matches("-compatible"));
            }
            word
        });
        for i in generators {
            elements.entry(i).or_insert((0, 0)).1 = floor as u32;
        }
        for i in chips {
            elements.entry(i).or_insert((0, 0)).0 = floor as u32;
        }
    }

    let h = |state: u32| -> u32 {
        (0..ITEMS).map(|x| 0b11 - (state >> x * 2 & 0b11)).sum::<u32>() / 2
    };

    let mut elements: Vec<_> = elements.values().copied().collect();
    elements.sort_unstable();

    let start = elements.into_iter().enumerate().fold(0, |acc, (n, floors)| {
        acc | floors.1 << n * 4 | floors.0 << n * 4 + 2
    });

    let end = (0..ITEMS + 1).fold(0, |acc, x| acc | 0b11 << x * 2);

    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let mut f_scores = HashMap::new();
    f_scores.insert(start, h(start));

    let mut open_set = BTreeSet::new();
    open_set.insert((f_scores[&start], Reverse(g_scores[&start]), start));

    let mut neighbors = Vec::new();

    while let Some(&current) = open_set.iter().next() {
        open_set.remove(&current);
        let current = current.2;
        if current == end {
            println!("{}", g_scores[&current]);
            break;
        }

        let current_g_score = g_scores[&current];
        
        get_neighbors(current, &mut neighbors);

        for &neighbor in &neighbors {
            let tentative_g_score = current_g_score + 1;
            let neighbor_g_score = g_scores.get(&neighbor).copied().unwrap_or(u32::MAX);

            if tentative_g_score < neighbor_g_score {
                if let Some(old_f_score) = f_scores.get(&neighbor).copied() {
                    open_set.remove(&(old_f_score, Reverse(neighbor_g_score), neighbor));
                }
                g_scores.insert(neighbor, tentative_g_score);
                let new_f_score = tentative_g_score + h(neighbor);
                f_scores.insert(neighbor, new_f_score);
                open_set.insert((new_f_score, Reverse(tentative_g_score), neighbor));
            }
        }

        neighbors.clear();
    }
}
