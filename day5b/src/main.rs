use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    let orig_len = input.len();

    let mut password = vec![None; 8];

    for i in 0.. {
        input.append(&mut format!("{}", i).into_bytes());
        let hash = md5::compute(&input);
        if hash[0] == 0 && hash[1] == 0 && hash[2] <= 7 && password[hash[2] as usize].is_none() {
            password[hash[2] as usize] = Some(hash[3] >> 4);
            // println!("{}", password.iter().map(|&x| {
            //     if let Some(num) = x {
            //         std::char::from_digit(num as u32, 16).unwrap()
            //     } else {
            //         '_'
            //     }
            // }).collect::<String>());
            if password.iter().all(|x| x.is_some()) {
                break;
            }
        }
        input.truncate(orig_len);
    }

    println!(
        "{}", password.into_iter().map(|x| std::char::from_digit(x.unwrap() as u32, 16).unwrap()).collect::<String>()
    );
}
