const H: i8 = 4;
const W: i8 = 4;
const END: (i8, i8) = (W - 1, H - 1);

fn longest(code: &mut String, path: &mut String, x: i8, y: i8) -> Option<usize> {
    let len = code.len();
    code.extend(path.chars());
    let hash = md5::compute(&*code);
    code.truncate(len);
    [
        (0, -1, 'U'),
        (0, 1, 'D'),
        (-1, 0, 'L'),
        (1, 0, 'R'),
    ].iter()
     .zip(hash.into_iter().flat_map(|&x| [x >> 4 & 0xF, x & 0xF].iter().copied().collect::<Vec<_>>()))
     .filter_map(|(&(dx, dy, dir), status)| {
        let x = x + dx;
        let y = y + dy;
        if status < 0xB || x < 0 || y < 0 || x >= W || y >= H {
            None
        } else if (x, y) == END {
            Some(path.len() + 1)
        } else {
            path.push(dir);
            let this_longest = longest(code, path, x, y);
            path.pop();
            this_longest
        }
    }).max()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut path = String::new();
    println!("{}", longest(&mut input, &mut path, 0, 0).unwrap());
}
