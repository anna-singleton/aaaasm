#[cfg(test)]
use crate::parser::*;

#[test]
fn parse_one_number_test() {
    let ops = parse_operands(&vec!["101"]);
    assert_eq!(ops, Ok(vec![Operand::Number(101)]));
}

#[test]
fn parse_one_register_test() {
    let ops = parse_operands(&vec!["r2"]);
    assert_eq!(ops, Ok(vec![Operand::Register(2)]));
}

#[test]
fn parse_multiple_test() {
    let ops = parse_operands(&vec!["r2", "123", "r5", "109"]);
    assert_eq!(ops, Ok(vec![Operand::Register(2), Operand::Number(123),
                            Operand::Register(5), Operand::Number(109)]));
}

#[test]
fn parse_register_fail() {
    let ops = parse_operands(&vec!["r"]);
    assert!(ops.is_err());
}

#[test]
fn parse_register_fail2() {
    let ops = parse_operands(&vec!["rfoo"]);
    assert!(ops.is_err());
}

#[test]
fn parse_operand_fail() {
    let ops = parse_operands(&vec!["bar"]);
    assert!(ops.is_err());
}

#[test]
fn parse_instruction1_test() {
    let input = "NOOP";
    let goal = Ok(Instruction::NOOP());
    assert_eq!(parse_instruction(input), goal);
}

#[test]
fn parse_instruction2_test() {
    let input = "M2R_LOAD 200 R2";
    let goal = Ok(Instruction::M2R_LOAD(200, 2));
    assert_eq!(parse_instruction(input), goal);
}
