use crate::instruction::Opcode;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod register_parsers;
pub mod program_parsers;
pub mod opcode;
pub mod instruction_parsers;
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    IntegerOperand{value: i32},
    Number{value: i32}
}
