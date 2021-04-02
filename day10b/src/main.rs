use std::io::Read;

#[derive(Clone)]
enum Target {
    Bot(u8),
    Output(u8),
}

struct Bot {
    values: Vec<u8>,
    targets: Vec<Target>,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut start_values = Vec::new();
    let mut bots = std::collections::HashMap::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();
        if words.next().unwrap() == "value" {
            let value: u8 = words.next().unwrap().parse().unwrap();
            let bot: u8 = words.nth(3).unwrap().parse().unwrap();
            start_values.push((value, bot));
        } else {
            let bot: u8 = words.next().unwrap().parse().unwrap();
            let low_target_type = words.nth(3).unwrap();
            let low_target: u8 = words.next().unwrap().parse().unwrap();
            let high_target_type = words.nth(3).unwrap();
            let high_target: u8 = words.next().unwrap().parse().unwrap();

            bots.insert(bot, Bot {
                values: Vec::new(),
                targets: vec![
                    if low_target_type == "bot" { Target::Bot(low_target) } else { Target::Output(low_target) },
                    if high_target_type == "bot" { Target::Bot(high_target) } else { Target::Output(high_target) },
                ],
            });
        }
    }

    let mut ready_to_send = std::collections::HashSet::new();

    for (value, bot) in start_values {
        let values = &mut bots.get_mut(&bot).unwrap().values;
        values.push(value);
        if values.len() == 2 {
            ready_to_send.insert(bot);
        }
    }

    let mut send_iter = None;

    let mut chips = vec![u8::MAX; 3];

    'outer: loop {
        if send_iter.is_none() {
            send_iter = Some(ready_to_send.iter());
        }

        let bot_num = *send_iter.as_mut().unwrap().next().unwrap();
        let bot = bots.get_mut(&bot_num).unwrap();
        let targets = bot.targets.clone();
        bot.values.sort();

        if targets.iter().all(|x| {
            if let Target::Bot(target) = x {
                bots[target].values.len() < 2
            } else {
                true
            }
        }) {
            send_iter = None;

            for i in targets.into_iter().rev() {
                let val = bots.get_mut(&bot_num).unwrap().values.pop().unwrap();
                match i {
                    Target::Bot(target) => {
                        let values = &mut bots.get_mut(&target).unwrap().values;
                        values.push(val);
                        if values.len() >= 2 {
                            ready_to_send.insert(target);
                        }
                    }

                    Target::Output(target) => {
                        if target < 3 {
                            chips[target as usize] = val;
                            if chips.iter().all(|&x| x < u8::MAX) {
                                println!("{}", chips.into_iter().map(|x| x as u16).product::<u16>());
                                break 'outer;
                            }
                        }
                    }
                }
            }

            ready_to_send.remove(&bot_num);
        }
    }
}
