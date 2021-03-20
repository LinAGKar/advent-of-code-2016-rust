use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    let orig_len = input.len();

    let mut password = String::new();

    for i in 0.. {
        input.append(&mut format!("{}", i).into_bytes());
        let hash = md5::compute(&input);
        if hash[0] == 0 && hash[1] == 0 && hash[2] & 0xF0 == 0 {
            password.push(std::char::from_digit((hash[2] & 0xF) as u32, 16).unwrap());
            // println!("{}", password);
            if password.len() >= 8 {
                break;
            }
        }
        input.truncate(orig_len);
    }

    println!("{}", password);
}
