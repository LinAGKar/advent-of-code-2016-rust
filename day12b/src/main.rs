use std::io::Read;

enum Source {
    Val(i32),
    Reg(usize),
}

enum Instruction {
    CPY(Source, usize),
    INC(usize),
    DEC(usize),
    JNZ(Source, isize),
}

fn parse_reg(source: &str) -> usize {
    source.chars().next().unwrap() as usize - 'a' as usize
}

fn parse_source(source: &str) -> Source {
    if source.chars().all(|x| x.is_ascii_alphabetic()) {
        Source::Reg(parse_reg(source))
    } else {
        Source::Val(source.parse().unwrap())
    }
}

fn get_val(regs: &[i32; 4], source: &Source) -> i32 {
    match source {
        Source::Reg(x) => regs[*x],
        Source::Val(x) => *x,
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let instructions: Vec<_> = input.lines().map(|line| {
        let mut words = line.split_whitespace();
        match words.next().unwrap() {
            "cpy" => {
                let source = words.next().unwrap();
                let target = words.next().unwrap();
                Instruction::CPY(parse_source(source), parse_reg(target))
            }

            "inc" => {
                Instruction::INC(parse_reg(words.next().unwrap()))
            }

            "dec" => {
                Instruction::DEC(parse_reg(words.next().unwrap()))
            }

            "jnz" => {
                let source = words.next().unwrap();
                let offset = words.next().unwrap();
                Instruction::JNZ(parse_source(source), offset.parse().unwrap())
            }

            _ => { panic!("") }
        }
    }).collect();

    let mut regs = [0; 4];
    regs[2] = 1;
    let mut pc = 0;
    while let Some(instr) = instructions.get(pc) {
        pc += 1;

        match instr {
            Instruction::CPY(source, target) => {
                regs[*target] = get_val(&regs, source);
            }

            Instruction::INC(target) => {
                regs[*target] += 1;
            }

            Instruction::DEC(target) => {
                regs[*target] -= 1;
            }

            Instruction::JNZ(source, offset) => {
                if get_val(&regs, source) != 0 {
                    pc = (pc as isize - 1 + offset) as usize;
                }
            }
        }
    }

    println!("{}", regs[0]);
}
