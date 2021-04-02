use std::io::Read;

struct Bot {
    values: Vec<u8>,
    targets: Vec<Option<u8>>,
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
                    if low_target_type == "bot" { Some(low_target) } else { None },
                    if high_target_type == "bot" { Some(high_target) } else { None },
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

    loop {
        if send_iter.is_none() {
            send_iter = Some(ready_to_send.iter());
        }
        let bot_num = *send_iter.as_mut().unwrap().next().unwrap();
        let bot = bots.get_mut(&bot_num).unwrap();
        let targets = bot.targets.clone();
        bot.values.sort();
        if bot.values == [17, 61] {
            println!("{}", bot_num);
            break;
        }
        if targets.iter().all(|x| {
            if let Some(target) = x {
                bots[target].values.len() < 2
            } else {
                true
            }
        }) {
            send_iter = None;

            for i in targets.into_iter().rev() {
                let val = bots.get_mut(&bot_num).unwrap().values.pop().unwrap();
                if let Some(target) = i {
                    let values = &mut bots.get_mut(&target).unwrap().values;
                    values.push(val);
                    if values.len() >= 2 {
                        ready_to_send.insert(target);
                    }
                }
            }

            ready_to_send.remove(&bot_num);
        }
    }
}
