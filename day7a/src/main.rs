use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();

    println!("{}", input.split(|&x| x == '\n' as u8).filter(|line| {
        line.windows(4).try_fold((false, false), |(in_bracket, abba), seq| {
            let in_bracket = (in_bracket || seq[0] == '[' as u8) && seq[0] != ']' as u8;
            let abba_now = seq[0] == seq[3] && seq[1] == seq[2] && seq[0] != seq[1];

            if !abba_now {
                Some((in_bracket, abba))
            } else if !in_bracket {
                Some((in_bracket, abba_now))
            } else {
                None
            }
        }).map(|(_, abba)| abba).unwrap_or(false)
    }).count());
}
