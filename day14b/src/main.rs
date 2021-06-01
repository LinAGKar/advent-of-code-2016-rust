use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();

    let mut triplets = VecDeque::new();
    let mut latest_quintet = vec![0; 'g' as usize];
    let mut count = 0;
    let mut hash = Vec::new();

    'outer: for i in 0.. {
        hash.extend(input.iter());
        hash.append(&mut format!("{}", i).into_bytes());

        for _ in 0..2017 {
            let digest = md5::compute(&hash);
            hash.clear();
            for &j in digest.iter() {
                for &k in &[j >> 4 & 0xF, j & 0xF] {
                    hash.push(if k < 10 {
                        '0' as u8 + k
                    } else {
                        'a' as u8 + k - 10
                    });
                }
            }
        }

        if let Some(num) = hash.windows(5).find_map(|x| {
            if x.iter().skip(1).all(|&y| y == x[0]) {
                Some(x[0])
            } else {
                None
            }
        }) {
            latest_quintet[num as usize] = i;
        }

        if let Some(num) = hash.windows(3).find_map(|x| {
            if x.iter().skip(1).all(|&y| y == x[0]) {
                Some(x[0])
            } else {
                None
            }
        }) {
            triplets.push_back((i, num));
        }

        while let Some(x) = triplets.pop_front() {
            let (j, num) = x;
            if j + 1000 > i {
                triplets.push_front(x);
                break;
            }
            if latest_quintet[num as usize] > j {
                count += 1;
                if count >= 64 {
                    println!("{}", j);
                    break 'outer;
                }
            }
        }

        hash.clear();
    }
}
