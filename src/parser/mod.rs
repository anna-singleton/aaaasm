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

    match words[0] {
        "NOOP" => {

        }
    }

    todo!()
}
