use std::io::Read;

#[derive(Clone, Copy)]
enum Arg {
    Val(i32),
    Reg(usize),
}

#[derive(Clone, Copy)]
enum Instruction {
    CPY(Arg, Arg),
    INC(Arg),
    DEC(Arg),
    JNZ(Arg, Arg),
    TGL(Arg),
}

fn parse_reg(arg: &str) -> usize {
    arg.chars().next().unwrap() as usize - 'a' as usize
}

fn parse_arg(arg: &str) -> Arg {
    if arg.chars().all(|x| x.is_ascii_alphabetic()) {
        Arg::Reg(parse_reg(arg))
    } else {
        Arg::Val(arg.parse().unwrap())
    }
}

fn get_val(regs: &[i32; 4], arg: &Arg) -> i32 {
    match arg {
        Arg::Reg(x) => regs[*x],
        Arg::Val(x) => *x,
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut instructions: Vec<_> = input.lines().map(|line| {
        let mut words = line.split_whitespace();
        match words.next().unwrap() {
            "cpy" => {
                let source = words.next().unwrap();
                let target = words.next().unwrap();
                Instruction::CPY(parse_arg(source), parse_arg(target))
            }

            "inc" => {
                Instruction::INC(parse_arg(words.next().unwrap()))
            }

            "dec" => {
                Instruction::DEC(parse_arg(words.next().unwrap()))
            }

            "jnz" => {
                let source = words.next().unwrap();
                let offset = words.next().unwrap();
                Instruction::JNZ(parse_arg(source), parse_arg(offset))
            }

            "tgl" => {
                let source = words.next().unwrap();
                Instruction::TGL(parse_arg(source))
            }

            _ => { panic!("") }
        }
    }).collect();

    let mut regs = [0; 4];
    regs[0] = 12;
    let mut pc = 0;
    while let Some(instr) = instructions.get(pc) {
        pc += 1;

        match instr {
            Instruction::CPY(source, target) => {
                if let &Arg::Reg(reg) = target {
                    regs[reg] = get_val(&regs, source);
                }
            }

            Instruction::INC(target) => {
                if let &Arg::Reg(reg) = target {
                    regs[reg] += 1;
                }
            }

            Instruction::DEC(target) => {
                if let &Arg::Reg(reg) = target {
                    regs[reg] -= 1;
                }
            }

            Instruction::JNZ(source, offset) => {
                if get_val(&regs, source) != 0 {
                    pc = (pc as isize - 1 + get_val(&regs, offset) as isize) as usize;
                }
            }

            Instruction::TGL(source) => {
                let index = pc as isize - 1 + get_val(&regs, source) as isize;
                if index > 0 && (index as usize) < instructions.len() {
                    let index = index as usize;
                    instructions[index] = match instructions[index] {
                        Instruction::CPY(arg1, arg2) => Instruction::JNZ(arg1, arg2),
                        Instruction::INC(arg) => Instruction::DEC(arg),
                        Instruction::DEC(arg) => Instruction::INC(arg),
                        Instruction::JNZ(arg1, arg2) => Instruction::CPY(arg1, arg2),
                        Instruction::TGL(arg) => Instruction::INC(arg),
                    };
                }
            }
        }
    }

    println!("{}", regs[0]);
}
