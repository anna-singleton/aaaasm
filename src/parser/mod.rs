use crate::interpreter::Instruction;

#[derive(Debug, Clone)]
enum Operand {
    Register(i32), // like R1
    Number(i32) // like 102
}

impl std::str::FromStr for Operand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().nth(0).unwrap();
        if first == 'R' || first == 'r' {
            // register
            todo!()
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


    todo!()
}
