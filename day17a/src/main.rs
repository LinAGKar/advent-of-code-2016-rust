use std::collections::BinaryHeap;
use std::cmp::Reverse;

const H: i8 = 4;
const W: i8 = 4;

fn h(x: i8, y: i8) -> u8 {
    H as u8 - 1 - y as u8 + W as u8 - 1 - x as u8
}

fn add_neighbors(
    code: &mut String,
    path: &str,
    x: i8,
    y: i8,
    open_set: &mut BinaryHeap<(Reverse<u8>, i8, i8, String)>,
) {
    let len = code.len();
    code.extend(path.chars());
    let hash = md5::compute(&*code);
    for (&(dx, dy, dir), status) in [
        (0, -1, 'U'),
        (0, 1, 'D'),
        (-1, 0, 'L'),
        (1, 0, 'R'),
    ].iter().zip(hash.into_iter().flat_map(|&x| [x >> 4 & 0xF, x & 0xF].iter().copied().collect::<Vec<_>>())) {
        let x = x + dx;
        let y = y + dy;
        if status >= 0xb && x >= 0 && y >= 0 && x < W && y < H {
            let new_path: String = path.chars().chain([dir].iter().copied()).collect();
            open_set.push((
                Reverse(h(x, y) + new_path.len() as u8),
                x, y,
                new_path,
            ));
        }
    }
    code.truncate(len);
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let end = (W - 1, H - 1);

    let mut open_set = BinaryHeap::new();
    open_set.push((Reverse(h(0, 0)), 0, 0, "".to_string()));

    while let Some((_, x, y, path)) = open_set.pop() {
        if (x, y) == end {
            println!("{}", path);
            break;
        }

        add_neighbors(&mut input, &path, x, y, &mut open_set);
    }
}
