use crate::interpreter::Instruction;
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operand {
    Register(i32), // like R1
    Number(i32) // like 102
}

impl Operand {
    fn is_register(&self) -> bool {
        return match self {
            Operand::Register(_) => true,
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        return match self {
            Operand::Number(_) => true,
            _ => false,
        }
    }

    fn type_matches(&self, other: &Operand) -> bool {
        return std::mem::discriminant(self) == std::mem::discriminant(other);
    }

    fn inner(&self) -> i32 {
        return match self {
            Operand::Register(x) => *x,
            Operand::Number(x) => *x,
        };
    }
}

impl std::str::FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().nth(0).unwrap();
        if first == 'R' || first == 'r' {
            // register
            if s.len() == 1 {
                return Err("Attempted to parse register, but no register \
                           number was given!".to_string())
            }
            return match (&s[1..]).parse::<i32>() {
                Ok(num) => Ok(Operand::Register(num)),
                Err(err) => Err(format!("Attempted to parse {} as register number but failed! \
                                        Error given: {}", &s[1..], err.to_string())),
            };
        } else {
            // other number
            let number = s.parse::<i32>();
            if number.is_err() {
                return Err(number.unwrap_err().to_string());
            }
            return Ok(Operand::Number(number.unwrap()));
        }
    }
}

fn parse_operands(operands: &Vec<&str>) -> Result<Vec<Operand>, String> {
    let ops:Vec<_> = operands.iter().map(|word| word.parse::<Operand>()).collect();
    for op in ops.iter() {
        if op.is_err() {
            return Err(op.clone().unwrap_err());
        }
    }
    return Ok(ops.into_iter().map(|op| op.unwrap()).collect());
}

pub fn parse_instruction(s: &str) -> Result<Instruction, String> {
    let words:Vec<_> = s.split(' ').collect();

    if words.is_empty() {
        panic!("parse instruction was passed a blank line.");
    }

    let ops = parse_operands(&words[1..].to_vec());

    if ops.is_err()
    {
        return Err(ops.unwrap_err());
    }

    let ops = ops.unwrap();


    let mut instruction = Instruction::NOOP();

    let arg_fmt = match words[0] {
        "NOOP" => {
            vec![]
        }
        "LOAD" => {
            instruction = Instruction::LOAD(0);
            vec![Operand::Number(0)]
        }
        "R2A_LOAD" => {
            instruction = Instruction::R2A_LOAD(0);
            vec![Operand::Register(0)]
        }
        "M2R_LOAD" => {
            instruction = Instruction::M2R_LOAD(0, 0);
            vec![Operand::Number(0), Operand::Register(0)]
        }
        "M2A_LOAD" => {
            instruction = Instruction::M2A_LOAD(0);
            vec![Operand::Number(0)]
        }
        "A2R_STORE" => {
            instruction = Instruction::A2R_STORE(0);
            vec![Operand::Register(0)]
        }
        "A2M_STORE" => {
            instruction = Instruction::A2M_STORE(0);
            vec![Operand::Number(0)]
        }
        "R2M_STORE" => {
            instruction = Instruction::R2M_STORE(0, 0);
            vec![Operand::Register(0), Operand::Number(0)]
        }
        "I_ADD" => {
            instruction = Instruction::I_ADD(0);
            vec![Operand::Number(0)]
        }
        "JUMP" => {
            instruction = Instruction::JUMP(0);
            vec![Operand::Number(0)]
        }
        "JUMP_NEG" => {
            instruction = Instruction::JUMP_NEG(0);
            vec![Operand::Number(0)]
        }
        _ => {
            return Err(format!("Illegal Instruction: {} does not match any known instruction", words[0]).to_string())
        }
    };

    if !matching_operand_formats(&ops, &arg_fmt) {
        return Err(format!("Bad arguments passed to {}", words[0]).to_string());
    }

    return Ok(match instruction {
        Instruction::NOOP() => instruction,
        Instruction::LOAD(_) => Instruction::LOAD(ops[0].inner()),
        Instruction::R2A_LOAD(_) => Instruction::R2A_LOAD(ops[0].inner()),
        Instruction::M2R_LOAD(_, _) => Instruction::M2R_LOAD(ops[0].inner(), ops[1].inner()),
        Instruction::M2A_LOAD(_) => Instruction::M2A_LOAD(ops[0].inner()),
        Instruction::A2R_STORE(_) => Instruction::A2R_STORE(ops[0].inner()),
        Instruction::A2M_STORE(_) => Instruction::A2M_STORE(ops[0].inner()),
        Instruction::R2M_STORE(_, _) => Instruction::R2M_STORE(ops[0].inner(), ops[1].inner()),
        Instruction::I_ADD(_) => Instruction::I_ADD(ops[0].inner()),
        Instruction::JUMP(_) => Instruction::JUMP(ops[0].inner()),
        Instruction::JUMP_NEG(_) => Instruction::JUMP_NEG(ops[0].inner()),
    });
}

fn matching_operand_formats(x: &Vec<Operand>, y: &Vec<Operand>) -> bool {
    if x.len() != y.len() {
        return false;
    }

    return x.iter().zip(y.iter()).all(|(op1, op2)| op1.type_matches(op2));
}
