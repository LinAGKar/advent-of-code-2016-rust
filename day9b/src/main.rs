fn decompressed_len(compressed: &str) -> u64 {
    let mut chars = compressed.trim().chars().enumerate().peekable();
    let mut count = 0;

    while let Some((n, next)) = chars.next() {
        if next == '(' {
            let x_pos = chars.find_map(|(m, x)| if x == 'x' { Some(m) } else { None }).unwrap();
            let rparen_pos = chars.find_map(|(m, x)| if x == ')' { Some(m) } else { None }).unwrap();
            let len: usize = compressed[n + 1..x_pos].parse().unwrap();
            let reps: u64 = compressed[x_pos + 1..rparen_pos].parse().unwrap();
            count += reps * decompressed_len(&compressed[rparen_pos + 1..=rparen_pos + len]);
            for _ in 0..len {
                chars.next();
            }
        } else {
            count += 1;
        }
    }

    count
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("{}", decompressed_len(input.trim()));
}
