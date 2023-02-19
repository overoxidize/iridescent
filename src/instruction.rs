#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Opcode {
    LOAD, // LOAD $0 #b: Loads the value of b into register $0.
    ADD, // ADD $0 $1 $2: Stores the sum of $0 and $1 into register $2.
    SUB, // SUB $0 $1 $2: Stores the difference of $0 and $1 into register $2.
    MUL, // MUL $0 $1 $2: Stores the product of $0 and $1 into register $2.
    DIV, // DIV $0 $1 $2: Stores the product of $0 and $1 into register $2.
    HLT, // Stops execution of current instruction.
    JMP, // JMP $0: Sets the program counter to $0, continuing execution from there.
    JMPF,// JMPF $0: Sets the program counter to pc + $0, continuing execution from there.
    JMPB, // JMPB $0: JMPF $0: Sets the program counter to pc - $0, continuing execution from there.
    EQ, // EQ $0 $1 $2: Compares the values of $0 and $2, setting `VM.equal_flag` to the results of the comparison.
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
    JNEQ,
    IGL, // Sends a request for an interrupt to the processor.
}

pub struct Instruction {
    // An instruction is a group of 32 bits, the first 8 of which, will be
    // an opcode, and the remaining ones will be up to three operands.
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode: opcode
        }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::LOAD,
            1 => return Opcode::ADD,
            2 => return Opcode::SUB,
            3 => return Opcode::MUL,
            4 => return Opcode::DIV,
            5 => return Opcode::HLT,
            6 => return Opcode::JMP,
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            9 => return Opcode::EQ,
            10 => return Opcode::NEQ,
            11 => return Opcode::GT,
            12 => return Opcode::LT,
            13 => return Opcode::GTQ,
            14 => return Opcode::LTQ,
            15 => return Opcode::JEQ,
            16 => return Opcode::JNEQ,
            _ => return Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    
    #[test]

    fn test_create_hlt() {
        let opcode = Opcode::HLT;

        assert_eq!(opcode, Opcode::HLT)
    }

    #[test]

    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT)
    }
}