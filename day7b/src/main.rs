use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();

    println!("{}", input.split(|&x| x == '\n' as u8).filter(|line| {
        line.windows(3).try_fold((false, HashSet::new(), HashSet::new()), |(in_bracket, mut abas, mut babs), seq| {
            let in_bracket = (in_bracket || seq[0] == '[' as u8) && seq[0] != ']' as u8;
            if seq[0] == seq[2] && seq[0] != seq[1] {
                if in_bracket {
                    let bab = (seq[1], seq[0]);
                    if abas.contains(&bab) {
                        return None;
                    }
                    babs.insert(bab);
                } else {
                    let aba = (seq[0], seq[1]);
                    if babs.contains(&aba) {
                        return None;
                    }
                    abas.insert(aba);
                }
            }

            Some((in_bracket, abas, babs))
        }).is_none()
    }).count());
}
