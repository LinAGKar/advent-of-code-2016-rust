fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let (x, y, _, _) = input.trim().split(", ").fold((0, 0, 1, 0), |(x, y, dx, dy), instruction| {  
        let distance: i16 = instruction[1..].parse().unwrap();
        let (dx, dy) = match &instruction[..1] {
            "R" => (dy, -dx),
            "L" => (-dy, dx),
            _ => { panic!(""); },
        };
        (x + dx * distance, y + dy * distance, dx, dy)
    });
    println!("{}", x.abs() + y.abs());
}
