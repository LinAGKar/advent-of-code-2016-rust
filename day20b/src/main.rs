use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut rules: Vec<_> = input.lines().map(|line| {
        let mut numbers = line.split('-').map(|x| x.parse::<u64>().unwrap());
        let min = numbers.next().unwrap();
        (min, numbers.next().unwrap())
    }).chain([(u32::MAX as u64 + 1, u64::MAX - 1)].iter().copied()).collect();

    rules.sort();

    println!("{}", rules.into_iter().fold((0, 0), |(curr, count), (min, max)| {
        if curr < min {
            (max + 1, count + min - curr)
        } else if curr <= max {
            (max + 1, count)
        } else {
            (curr, count)
        }
    }).1);
}
