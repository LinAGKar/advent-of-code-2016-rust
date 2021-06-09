use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut rules: Vec<_> = input.lines().map(|line| {
        let mut numbers = line.split('-').map(|x| x.parse::<u32>().unwrap());
        let min = numbers.next().unwrap();
        (min, numbers.next().unwrap())
    }).collect();

    rules.sort();

    println!("{}", rules.into_iter().try_fold(0, |curr, (min, max)| {
        if curr < min {
            Err(curr)
        } else if curr <= max {
            Ok(max + 1)
        } else {
            Ok(curr)
        }
    }).unwrap_err());
}
