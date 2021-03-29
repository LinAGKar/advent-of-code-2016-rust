fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut chars = input.trim().chars().enumerate().peekable();
    let mut count = 0;
    while let Some((n, next)) = chars.next() {
        if next == '(' {
            let x_pos = chars.find_map(|(m, x)| if x == 'x' { Some(m) } else { None }).unwrap();
            let rparen_pos = chars.find_map(|(m, x)| if x == ')' { Some(m) } else { None }).unwrap();
            let len: usize = input[n + 1..x_pos].parse().unwrap();
            let reps: usize = input[x_pos + 1..rparen_pos].parse().unwrap();
            count += len * reps;
            for _ in 0..len {
                chars.next();
            }
        } else {
            count += 1;
        }
    }

    println!("{}", count);
}
