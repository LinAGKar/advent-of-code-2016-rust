use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap};
use std::io::Read;

#[derive(PartialEq)]
enum Tile {
    Wall,
    Floor,
}

fn h(pos: (isize, isize), goal: (isize, isize)) -> isize {
    (pos.0 - goal.0).abs() + (pos.1 - goal.1).abs()
}

fn a_star(field: &Vec<Vec<Tile>>, start: (isize, isize), goal: (isize, isize)) -> Option<isize> {
    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let mut f_scores = HashMap::new();
    f_scores.insert(start, h(start, goal));

    let mut open_set = BTreeSet::new();
    open_set.insert((f_scores[&start], Reverse(0), start));

    while let Some(&current) = open_set.iter().next() {
        open_set.remove(&current);

        let (_, _, (x, y)) = current;
        let g_score = g_scores[&(x, y)];

        if (x, y) == goal {
            return Some(g_score);
        }

        for &pos in &[
            (x, y + 1),
            (x, y - 1),
            (x + 1, y),
            (x - 1, y),
        ] {
            if field[pos.1 as usize][pos.0 as usize] == Tile::Wall {
                continue;
            }

            let tentative_g_score = g_score + 1;
            let g_score = g_scores.get(&pos).copied();

            if tentative_g_score < g_score.unwrap_or(isize::MAX) {
                if let Some(g_score) = g_score {
                    open_set.remove(&(f_scores[&pos], Reverse(g_score), pos));
                }

                let f_score = tentative_g_score + h(pos, goal);
                f_scores.insert(pos, f_score);
                g_scores.insert(pos, tentative_g_score);

                open_set.insert((f_score, Reverse(tentative_g_score), pos));
            } 
        }
    }

    None
}

fn min_dist(distances: &Vec<Vec<isize>>, used: &mut Vec<bool>, curr: usize, dist: isize) -> isize {
    (1..distances.len()).filter_map(|i| {
        if used[i] {
            None
        } else {
            used[i] = true;
            let result = Some(min_dist(distances, used, i, dist + distances[curr][i]));
            used[i] = false;
            result
        }
    }).min().unwrap_or(dist + distances[curr][0])
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut waypoints = Vec::new();
    let field: Vec<Vec<_>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, tile)| {
            if tile == '#' {
                Tile::Wall
            } else {
                if let Some(num) = tile.to_digit(10) {
                    let num = num as usize;
                    if waypoints.len() <= num {
                        waypoints.resize(num + 1, (-1, -1));
                    }
                    waypoints[num] = (x as isize, y as isize);
                }
                Tile::Floor
            }
        }).collect()
    }).collect();

    let distances: Vec<Vec<_>> = waypoints.iter().copied().enumerate().map(|(n, i)| {
        waypoints.iter().skip(n + 1).copied().map(|j| {
            a_star(&field, i, j).unwrap()
        }).collect()
    }).collect();

    let distances: Vec<Vec<_>> = distances.iter().enumerate().map(|(n, a)| {
        (0..n).map(|b| {
            distances[b][n - b - 1]
        }).chain([0].iter().copied()).chain(a.iter().copied()).collect()
    }).collect();

    let mut used = vec![false; distances.len()];
    println!("{}", min_dist(&distances, &mut used, 0, 0));
}
