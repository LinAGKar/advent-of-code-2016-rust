fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let (x, y) = input.trim().split(", ").try_fold(
        (0, 0, 1, 0, [(0, 0)].iter().cloned().collect::<std::collections::HashSet<_>>()),
        |(x, y, dx, dy, mut visited), instruction| {  
            let distance: i16 = instruction[1..].parse().unwrap();
            let (dx, dy) = match &instruction[..1] {
                "R" => (dy, -dx),
                "L" => (-dy, dx),
                _ => { panic!(""); },
            };
            if let Some(pos) = (1..=distance).find_map(|i| visited.replace((x + dx * i, y + dy * i))) {
                Err(pos)
            } else {
                let (x, y) = (x + dx * distance, y + dy * distance);
                Ok((x, y, dx, dy, visited))
            }
        },
    ).unwrap_err();

    println!("{}", x.abs() + y.abs());
}
