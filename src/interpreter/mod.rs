#![allow(non_snake_case)]
mod tests;

const REG_NUMBER:usize = 4;
const MEM_SIZE:usize = 1024;

// bool is whether or not to increase the PC
type InstructionReturn = Result<bool, ()>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    NOOP(),
    LDAI(i32),
    ADDI(i32),
    JMPA(usize),
    JMPN(usize),
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    instructions: Vec<Instruction>,
    pc: usize,
    accumulator: i32,
    registers: [i32; REG_NUMBER]
}

impl State {
    pub fn init(ins: Vec<Instruction>) -> State {
        State {
            instructions: ins,
            pc: 0,
            accumulator: 0,
            registers: [0; REG_NUMBER],

        }
    }

    pub fn run(&mut self) -> InstructionReturn {
        let ret = match self.instructions.get(self.pc) {
            Some(ins) => {
                match ins {
                    Instruction::NOOP() => Ok(true),
                    Instruction::LDAI(x) => LDAI(self, *x),
                    Instruction::ADDI(x) => ADDI(self, *x),
                    Instruction::JMPA(x) => JMPA(self, *x),
                    Instruction::JMPN(x) => JMPN(self, *x),
                }
            },
            None => {
                eprintln!("Attempted to execute instruction {}, but that is out of bounds!", self.pc);
                return Err(())
            },
        };

        if ret.is_ok() {
            if ret.unwrap() {
                self.pc += 1;
            }
        }
        return ret
    }
}


fn LDAI(s: &mut State, x: i32) -> InstructionReturn {
    s.accumulator += x;
    return Ok(true);
}

fn ADDI(s: &mut State, x: i32) -> InstructionReturn {
    s.accumulator += x;
    return Ok(true);
}

fn JMPA(s: &mut State, x: usize) -> InstructionReturn {
    if x >= s.instructions.len() {
        // illegal jump
        eprintln!("Illegal jump action. Tried to jump from {} to {} but the \
            last instruction has an idx of {}", s.pc, x,
            s.instructions.len());
        return Err(());
    }

    s.pc = x;
    return Ok(false)
}

fn JMPN(s: &mut State, x: usize) -> InstructionReturn {
    if x >= s.instructions.len() {
        // illegal jump
        eprintln!("Illegal jump action. Tried to jump from {} to {} but the \
            last instruction has an idx of {}", s.pc, x,
            s.instructions.len());
        return Err(());
    }
    if s.accumulator >= 0 {
        return Ok(true)
    }

    s.pc = x;
    return Ok(false)
}
