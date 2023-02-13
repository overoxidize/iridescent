use crate::instruction::Opcode;

pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
}
