#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod tests;

const REG_NUMBER:usize = 4;
const MEM_SIZE:usize = 1024;

// bool is whether or not to increase the PC
type InstructionReturn = Result<bool, ()>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instruction {
    NOOP(),
    LOAD(i32), // LOAD IMMEDIATE INTO ACC
    R2A_LOAD(i32), // LOAD FROM REG INTO ACC
    M2R_LOAD(i32, i32), // LOAD FROM MEMORY TO REG
    M2A_LOAD(i32), // LOAD FROM MEMORY TO ACC
    A2R_STORE(i32), // STORE FROM ACC INTO REG
    A2M_STORE(i32), // STORE FROM ACC INTO MEM
    R2M_STORE(i32, i32), // STORE FROM REG INTO MEM
    I_ADD(i32), // ADD IMMEDIATE TO ACC
    JUMP(i32), // ALWAYS JUMP TO IMMEDIATE
    JUMP_NEG(i32), // JUMP TO IMMEDIATE IF ACC < 0
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    instructions: Vec<Instruction>,
    pc: usize,
    accumulator: i32,
    registers: [i32; REG_NUMBER],
    memory: [i32; MEM_SIZE]
}

impl State {
    pub fn init(ins: Vec<Instruction>) -> State {
        State {
            instructions: ins,
            pc: 0,
            accumulator: 0,
            registers: [0; REG_NUMBER],
            memory: [0; MEM_SIZE],
        }
    }

    pub fn run(&mut self) -> InstructionReturn {
        let ret = match self.instructions.get(self.pc) {
            Some(ins) => {
                match ins {
                    // special instructions
                    Instruction::NOOP() => Ok(true),

                    // load instructions
                    Instruction::LOAD(x) => LOAD(self, *x),
                    Instruction::R2A_LOAD(reg) => R2A_LOAD(self, *reg),
                    Instruction::M2R_LOAD(mem_addr, reg) => M2R_LOAD(self, *mem_addr, *reg),
                    Instruction::M2A_LOAD(mem_addr) => M2A_LOAD(self, *mem_addr),

                    // store instructions
                    Instruction::A2R_STORE(reg) => A2R_STORE(self, *reg),
                    Instruction::A2M_STORE(mem_addr) => A2M_STORE(self, *mem_addr),
                    Instruction::R2M_STORE(reg, mem_addr) => R2M_STORE(self, *reg, *mem_addr),

                    // maths instructions
                    Instruction::I_ADD(x) => I_ADD(self, *x),

                    // jump instructions
                    Instruction::JUMP(ins) => JUMP(self, *ins),
                    Instruction::JUMP_NEG(ins) => JUMP_NEG(self, *ins),
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


fn LOAD(s: &mut State, x: i32) -> InstructionReturn {
    s.accumulator += x;
    return Ok(true);
}

fn R2A_LOAD(s: &mut State, x: i32) -> InstructionReturn {
    if x < 0 {
        eprintln!("Illegal register access! Attempted to access {} but negative indices not allowed", x);
        return Err(())
    }
    else if x < REG_NUMBER as i32 {
        s.accumulator = s.registers[x as usize];
        return Ok(true);
    } else {
        eprintln!("Illegal register access! Attempted to access {} but there is only {} registers", x, REG_NUMBER);
        return Err(())
    }
}

fn M2R_LOAD(s: &mut State, mem_addr: i32, reg: i32) -> InstructionReturn {
    if mem_addr < 0 || mem_addr >= MEM_SIZE as i32 {
        eprintln!("Attempted to access memory out of bounds! Accessed {} but the mem size is {}",
        mem_addr, MEM_SIZE);
        return Err(())
    }
    if reg < 0 || reg >= REG_NUMBER as i32 {
        eprintln!("Attempted to access bad register! Accessed {} but the register amount is {}",
        reg, REG_NUMBER);
        return Err(())
    }

    s.registers[reg as usize] = s.memory[mem_addr as usize];
    return Ok(true);
}

fn M2A_LOAD(s: &mut State, mem_addr: i32) -> InstructionReturn {
    if mem_addr < 0 || mem_addr >= MEM_SIZE as i32 {
        eprintln!("Attempted to access memory out of bounds! Accessed {} but the mem size is {}",
        mem_addr, MEM_SIZE);
        return Err(())
    }
    s.accumulator = s.memory[mem_addr as usize];

    return Ok(true);
}

fn A2R_STORE(s: &mut State, reg: i32) -> InstructionReturn {
    if reg < 0 || reg >= REG_NUMBER as i32 {
        eprintln!("Attempted to access bad register! Accessed {} but the register amount is {}",
        reg, REG_NUMBER);
        return Err(())
    }
    s.registers[reg as usize] = s.accumulator;
    return Ok(true)
}

fn A2M_STORE(s: &mut State, mem_addr: i32) -> InstructionReturn {
    if mem_addr < 0 || mem_addr >= MEM_SIZE as i32 {
        eprintln!("Attempted to access memory out of bounds! Accessed {} but the mem size is {}",
        mem_addr, MEM_SIZE);
        return Err(())
    }
    s.memory[mem_addr as usize] = s.accumulator;

    return Ok(true);
}

fn R2M_STORE(s: &mut State, reg: i32, mem_addr: i32) -> InstructionReturn {
    if mem_addr < 0 || mem_addr >= MEM_SIZE as i32 {
        eprintln!("Attempted to access memory out of bounds! Accessed {} but the mem size is {}",
        mem_addr, MEM_SIZE);
        return Err(())
    }
    if reg < 0 || reg >= REG_NUMBER as i32 {
        eprintln!("Attempted to access bad register! Accessed {} but the register amount is {}",
        reg, REG_NUMBER);
        return Err(())
    }
    s.memory[mem_addr as usize] = s.registers[reg as usize];

    return Ok(true);
}

fn I_ADD(s: &mut State, x: i32) -> InstructionReturn {
    s.accumulator += x;
    return Ok(true);
}

fn JUMP(s: &mut State, x: i32) -> InstructionReturn {
    if x < 0 {
        eprintln!("Illegal jump action. Tried to jump to {}", x);
        return Err(())
    }
    if x >= s.instructions.len() as i32 {
        // illegal jump
        eprintln!("Illegal jump action. Tried to jump from {} to {} but the \
            last instruction has an idx of {}", s.pc, x,
            s.instructions.len());
        return Err(());
    }

    s.pc = x as usize;
    return Ok(false)
}

fn JUMP_NEG(s: &mut State, x: i32) -> InstructionReturn {
    if x < 0 {
        eprintln!("Illegal jump action. Tried to jump to {}", x);
        return Err(())
    }
    if x >= s.instructions.len() as i32 {
        // illegal jump
        eprintln!("Illegal jump action. Tried to jump from {} to {} but the \
            last instruction has an idx of {}", s.pc, x,
            s.instructions.len());
        return Err(());
    }
    if s.accumulator >= 0 {
        return Ok(true)
    }

    s.pc = x as usize;
    return Ok(false)
}
