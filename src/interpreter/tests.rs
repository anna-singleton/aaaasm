#[cfg(test)]
use crate::interpreter::*;

#[test]
fn noop_test() {
    let mut state = State::init(vec![Instruction::NOOP()]);
    assert_eq!(state.run(), Ok(true));
}

#[test]
fn ldai_test() {
    let mut state = State::init(vec![Instruction::LDAI(5)]);
    assert_eq!(state.run(), Ok(true));
    assert_eq!(state.accumulator, 5);
}

#[test]
fn addi_test() {
    let mut state = State::init(vec![Instruction::ADDI(10)]);
    assert_eq!(state.run(), Ok(true));
    assert_eq!(state.accumulator, 10);
}

#[test]
fn jmpa_ok_test() {
    let mut state = State::init(vec![Instruction::JMPA(2), Instruction::NOOP(), Instruction::NOOP()]);
    assert_eq!(state.run(), Ok(false));
    assert_eq!(state.pc, 2);
}

#[test]
fn jmpa_err_test() {
    let mut state = State::init(vec![Instruction::JMPA(10)]);
    assert_eq!(state.run(), Err(()));
}

#[test]
fn jmpn_ok_test() {
    let mut state = State::init(vec![Instruction::JMPN(2), Instruction::NOOP(), Instruction::NOOP()]);
    state.accumulator = -1;
    assert_eq!(state.run(), Ok(false));
    assert_eq!(state.pc, 2);
}

#[test]
fn jmpn_nojmp_test() {
    let mut state = State::init(vec![Instruction::JMPN(2), Instruction::NOOP(), Instruction::NOOP()]);
    state.accumulator = 1;
    assert_eq!(state.run(), Ok(true));
    assert_eq!(state.pc, 1);
}

#[test]
fn jmpn_err_test() {
    let mut state = State::init(vec![Instruction::JMPN(10)]);
    assert_eq!(state.run(), Err(()));
}

