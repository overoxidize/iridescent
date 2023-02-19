use nom::types::CompleteStr;

use assembler::instruction_parsers::{AssemblerInstruction, instruction_one};

#[derive(Debug, PartialEq)]

pub struct Program {
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