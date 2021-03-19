use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let re = regex::Regex::new(r"(?m)^([a-z-]+)-(\d+)\[([a-z]+)\]$").unwrap();

    println!("{}", re.captures_iter(&input).find_map(|line| {
        let mut counts = std::collections::HashMap::new();
        for i in line[1].chars().filter(|x| x.is_ascii_lowercase()) {
            *counts.entry(i).or_insert(0u8) += 1;
        }
        let mut counts: Vec<_> = counts.into_iter().map(|(letter, count)| (std::cmp::Reverse(count), letter)).collect();
        counts.sort_unstable();
        if counts.into_iter().take(5).map(|(_, letter)| letter).collect::<String>() == line[3] || true {
            let id = line[2].parse::<u32>().unwrap();
            let decrypted = line[1].chars().map(|letter| {
                if letter.is_ascii_lowercase() {
                    const START: u32 = 'a' as u32;
                    const LEN: u32 = 'z' as u32 + 1 - START;
                    std::char::from_u32((letter as u32 - START + id) % LEN + START).unwrap()
                } else {
                    ' '
                }
            }).collect::<String>();
            if decrypted == "northpole object storage" {
                Some(id)
            } else {
                None
            }
        } else {
            None
        }
    }).unwrap());
}
