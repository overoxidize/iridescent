#[derive(PartialEq)]
pub enum Opcode {
    LOAD, // LOAD $0 #b: Loads the value of b into register $0.
    ADD, // ADD $0 $1 $2: Stores the sum of $0 and $1 into register $2.
    SUB, // SUB $0 $1 $2: Stores the difference of $0 and $1 into register $2.
    MUL, // MUL $0 $1 $2: Stores the product of $0 and $1 into register $2.
    DIV, // DIV $0 $1 $2: Stores the product of $0 and $1 into register $2.
    HLT, // Stops execution of current instruction.
    JMP, // JMP $0: Sets the program counter to $0, continuing execution from there.
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
            0 => return Opcode::HLT,
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