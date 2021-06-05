fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut data: Vec<u8> = input.chars().map(|x| if x == '1' { 1 } else { 0 }).collect();

    const LEN: usize = 35651584;

    while data.len() < LEN {
        let b: Vec<_> = data.iter().copied().rev().map(|x| if x == 1 { 0 } else { 1 }).collect();
        data.push(0);
        data.extend(b);
    }

    data.truncate(LEN);

    while data.len() % 2 == 0 {
        data = data.chunks(2).map(|x| if x[0] == x[1] { 1 } else { 0 }).collect();
    }

    println!("{}", data.into_iter().map(|x| if x == 1 { '1' } else { '0' }).collect::<String>());
}
