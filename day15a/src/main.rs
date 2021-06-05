use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let disks: Vec<_> = input.lines().enumerate().map(|(n, line)| {
        let mut words = line.split_whitespace();
        let period: i32 = words.nth(3).unwrap().parse().unwrap();
        let offset: i32 = words.nth(7).unwrap().trim_matches('.').parse().unwrap();
        (period, (period * 2 - (n as i32 + 1) - offset) % period)
    }).collect();

    println!("{}", disks.into_iter().reduce(|(prev_period, prev_offset), (period, offset)| {
        let offset = ((offset - prev_offset) % period + period) % period;
        let offset = (0..).find(|x| (prev_period * x) % period == offset).unwrap();
        (prev_period * period, prev_period * offset + prev_offset)
    }).unwrap().1);
}
