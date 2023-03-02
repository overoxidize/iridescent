use nom::types::CompleteStr;
use nom::*;
use crate::assembler::instruction_parsers::{AssemblerInstruction, instruction_one};

#[derive(Debug, PartialEq, Clone)]

pub struct Program {
    /// Represents the program that will be fed into the VM, as a vector
    /// of Assembler instructions.`
    instructions: Vec<AssemblerInstruction>
}

named!(pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(instruction_one) >>
        (
            Program {
                instructions: instructions
            }
        )
    )
);

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instructions in &self.instructions {
            program.append(&mut instructions.to_bytes());
        }

        program
    }
}
#[cfg(tests)]
mod tests {
    
    #[test]
    fn test_parse_program() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, CompleteStr(""));
        assert_eq!(
            1,
            p.instructions.len()
        );
        // TODO: Figure out an ergonomic way to test the AssemblerInstruction returned
    }
    #[test]
    fn test_program_to_bytes() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}