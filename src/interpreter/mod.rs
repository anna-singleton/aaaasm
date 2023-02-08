#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod tests;

const REG_NUMBER:usize = 4;
const MEM_SIZE:usize = 1024;

// bool is whether or not to increase the PC
type InstructionReturn = Result<bool, String>;

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
    R_ADD(i32), // ADD REGISTER TO ACC
    JUMP(i32), // ALWAYS JUMP TO IMMEDIATE
    JUMP_NEG(i32), // JUMP TO IMMEDIATE IF ACC < 0
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Interpreter {
    instructions: Vec<Instruction>,
    pc: usize,
    pub accumulator: i32,
    registers: [i32; REG_NUMBER],
    memory: [i32; MEM_SIZE]
}

impl Interpreter {
    pub fn new(ins: Vec<Instruction>) -> Interpreter {
        Interpreter {
            instructions: ins,
            pc: 0,
            accumulator: 0,
            registers: [0; REG_NUMBER],
            memory: [0; MEM_SIZE],
        }
    }

    pub fn run_single(&mut self) -> Result<(), String> {
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
                    Instruction::R_ADD(x) => R_ADD(self, *x),

                    // jump instructions
                    Instruction::JUMP(ins) => JUMP(self, *ins),
                    Instruction::JUMP_NEG(ins) => JUMP_NEG(self, *ins),
                }
            },
            None => {
                return Err(format!("Attempted to execute instruction {}, but that is out of bounds!", self.pc));
            },
        };

        if ret.is_ok() {
            if *ret.as_ref().unwrap() {
                self.pc += 1;
            }
        }
        match ret {
            Ok(_) => return Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn run_program(&mut self) -> Result<i32, String> {
        while self.pc < self.instructions.len() {
            let result = self.run_single();
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }
        return Ok(self.accumulator);
    }
}


fn LOAD(s: &mut Interpreter, x: i32) -> InstructionReturn {
    s.accumulator = x;
    return Ok(true);
}

fn R2A_LOAD(s: &mut Interpreter, x: i32) -> InstructionReturn {
    if x < 0 {
        return Err(format!("Illegal register access! Attempted to access {} but negative indices not allowed", x));
    }
    else if x < REG_NUMBER as i32 {
        s.accumulator = s.registers[x as usize];
        return Ok(true);
    } else {
        return Err(format!("Illegal register access! Attempted to access {} but there is only {} registers", x, REG_NUMBER));
    }
}

fn M2R_LOAD(s: &mut Interpreter, mem_addr: i32, reg: i32) -> InstructionReturn {
    if mem_addr < 0 || mem_addr >= MEM_SIZE as i32 {
        return Err(format!("Attempted to access memory out of bounds! Accessed {} but the mem size is {}", mem_addr, MEM_SIZE))
    }
    if reg < 0 || reg >= REG_NUMBER as i32 {
        return Err(format!("Attempted to access bad register! Accessed {} but the register amount is {}", reg, REG_NUMBER))
    }

    s.registers[reg as usize] = s.memory[mem_addr as usize];
    return Ok(true);
}

fn M2A_LOAD(s: &mut Interpreter, mem_addr: i32) -> InstructionReturn {
    if mem_addr < 0 || mem_addr >= MEM_SIZE as i32 {
        return Err(format!("Attempted to access memory out of bounds! Accessed {} but the mem size is {}", mem_addr, MEM_SIZE))
    }
    s.accumulator = s.memory[mem_addr as usize];

    return Ok(true);
}

fn A2R_STORE(s: &mut Interpreter, reg: i32) -> InstructionReturn {
    if reg < 0 || reg >= REG_NUMBER as i32 {
        return Err(format!("Attempted to access bad register! Accessed {} but the register amount is {}", reg, REG_NUMBER));
    }
    s.registers[reg as usize] = s.accumulator;
    return Ok(true)
}

fn A2M_STORE(s: &mut Interpreter, mem_addr: i32) -> InstructionReturn {
    if mem_addr < 0 || mem_addr >= MEM_SIZE as i32 {
        return Err(format!("Attempted to access memory out of bounds! Accessed {} but the mem size is {}", mem_addr, MEM_SIZE))
    }
    s.memory[mem_addr as usize] = s.accumulator;

    return Ok(true);
}

fn R2M_STORE(s: &mut Interpreter, reg: i32, mem_addr: i32) -> InstructionReturn {
    if mem_addr < 0 || mem_addr >= MEM_SIZE as i32 {
        return Err(format!("Attempted to access memory out of bounds! Accessed {} but the mem size is {}", mem_addr, MEM_SIZE));
    }
    if reg < 0 || reg >= REG_NUMBER as i32 {
        return Err(format!("Attempted to access bad register! Accessed {} but the register amount is {}", reg, REG_NUMBER))
    }
    s.memory[mem_addr as usize] = s.registers[reg as usize];

    return Ok(true);
}

fn I_ADD(s: &mut Interpreter, x: i32) -> InstructionReturn {
    s.accumulator += x;
    return Ok(true);
}

fn R_ADD(s: &mut Interpreter, reg: i32) -> InstructionReturn {
    if reg < 0 || reg >= REG_NUMBER as i32 {
        return Err(format!("Attempted to access bad register! Accessed {} but the register amount is {}", reg, REG_NUMBER))
    }
    s.accumulator += s.registers[reg as usize];
    return Ok(true);
}

fn JUMP(s: &mut Interpreter, x: i32) -> InstructionReturn {
    if x < 0 {
        return Err(format!("Illegal jump action. Tried to jump to {}", x));
    }
    if x >= s.instructions.len() as i32 {
        // illegal jump
        return Err(format!("Illegal jump action. Tried to jump from {} to {} but the last instruction has an idx of {}", s.pc, x, s.instructions.len()));
    }

    s.pc = x as usize;
    return Ok(false)
}

fn JUMP_NEG(s: &mut Interpreter, x: i32) -> InstructionReturn {
    if x < 0 {
        return Err(format!("Illegal jump action. Tried to jump to {}", x));
    }
    if x >= s.instructions.len() as i32 {
        // illegal jump
        return Err(format!("Illegal jump action. Tried to jump from {} to {} but the last instruction has an idx of {}", s.pc, x, s.instructions.len()));
    }
    if s.accumulator >= 0 {
        return Ok(true)
    }

    s.pc = x as usize;
    return Ok(false)
}
