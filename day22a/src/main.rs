use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (mut by_used, mut by_avail): (Vec<_>, Vec<_>) = input.lines().skip(2).map(|line| {
        let mut words = line.split_whitespace();
        let used = words.nth(2).unwrap().trim_matches('T').parse::<u16>().unwrap();
        let avail = words.next().unwrap().trim_matches('T').parse::<u16>().unwrap();
        ((used, avail), avail)
    }).unzip();

    by_avail.sort_unstable();
    by_used.sort_unstable();

    let mut avail_iter = by_avail.iter().copied().enumerate();
    let mut curr_avail = avail_iter.next();
    let mut total = 0;

    for (used, used_avail) in by_used.into_iter().skip_while(|&(used, _)| used == 0) {
        while let Some(avail) = curr_avail {
            if avail.1 >= used {
                break;
            }
            curr_avail = avail_iter.next();
        }

        if let Some(avail) = curr_avail {
            total += by_avail.len() - avail.0;
            if used_avail >= used {
                total -= 1;
            }
        } else {
            break;
        }
    }

    println!("{}", total);
}
