#[cfg(test)]
use crate::interpreter::*;

#[test]
fn noop_test() {
    let mut state = Interpreter::new(vec![Instruction::NOOP()]);
    assert_eq!(state.run_single(), Ok(()));
}

#[test]
fn load_test() {
    let mut state = Interpreter::new(vec![Instruction::LOAD(5)]);
    assert_eq!(state.run_single(), Ok(()));
    assert_eq!(state.accumulator, 5);
}

#[test]
fn r2a_load_test() {
    let mut state = Interpreter::new(vec![Instruction::R2A_LOAD(0)]);
    state.registers[0] = 5;
    assert_eq!(state.run_single(), Ok(()));
    assert_eq!(state.accumulator, 5);
}

#[test]
fn m2r_load_test() {
    let mut state = Interpreter::new(vec![Instruction::M2R_LOAD(1, 2)]);
    state.memory[1] = 10;
    assert_eq!(state.run_single(), Ok(()));
    assert_eq!(state.registers[2], 10);
}

#[test]
fn m2a_load_test() {
    let mut state = Interpreter::new(vec![Instruction::M2A_LOAD(1)]);
    state.memory[1] = 10;
    assert_eq!(state.run_single(), Ok(()));
    assert_eq!(state.accumulator, 10);
}

#[test]
fn a2r_store_test() {
    let mut state = Interpreter::new(vec![Instruction::A2R_STORE(1)]);
    state.accumulator = 15;
    assert_eq!(state.run_single(), Ok(()));
    assert_eq!(state.registers[1], 15);
}

#[test]
fn a2m_store_test() {
    let mut state = Interpreter::new(vec![Instruction::A2M_STORE(5)]);
    state.accumulator = 20;
    assert_eq!(state.run_single(), Ok(()));
    assert_eq!(state.memory[5], 20);
}

#[test]
fn r2m_store_test() {
    let mut state = Interpreter::new(vec![Instruction::R2M_STORE(3, 50)]);
    state.registers[3] = 100;
    assert_eq!(state.run_single(), Ok(()));
    assert_eq!(state.memory[50], 100);
}

#[test]
fn i_add_test() {
    let mut state = Interpreter::new(vec![Instruction::I_ADD(10)]);
    assert_eq!(state.run_single(), Ok(()));
    assert_eq!(state.accumulator, 10);
}

#[test]
fn jump_ok_test() {
    let mut state = Interpreter::new(vec![Instruction::JUMP(2), Instruction::NOOP(), Instruction::NOOP()]);
    assert_eq!(state.run_single(), Ok(()));
    assert_eq!(state.pc, 2);
}

#[test]
fn jump_err_test() {
    let mut state = Interpreter::new(vec![Instruction::JUMP(10)]);
    assert!(state.run_single().is_err());
}

#[test]
fn jump_neg_ok_test() {
    let mut state = Interpreter::new(vec![Instruction::JUMP_NEG(2), Instruction::NOOP(), Instruction::NOOP()]);
    state.accumulator = -1;
    assert_eq!(state.run_single(), Ok(()));
    assert_eq!(state.pc, 2);
}

#[test]
fn jump_neg_nojmp_test() {
    let mut state = Interpreter::new(vec![Instruction::JUMP_NEG(2), Instruction::NOOP(), Instruction::NOOP()]);
    state.accumulator = 1;
    assert_eq!(state.run_single(), Ok(()));
    assert_eq!(state.pc, 1);
}

#[test]
fn jump_neg_err_test() {
    let mut state = Interpreter::new(vec![Instruction::JUMP_NEG(10)]);
    assert!(state.run_single().is_err());
}

