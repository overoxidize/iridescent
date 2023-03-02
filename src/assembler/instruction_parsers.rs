use crate::assembler::Token;
use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::integer_operand;
use crate::assembler::register_parsers::register;
use nom::named;
use nom::types::CompleteStr;
use nom::*;
#[derive(Debug, PartialEq, Clone)]
pub struct AssemblerInstruction {
    opcode: Token, 
    operand_1: Option<Token>,
    operand_2: Option<Token>,
    operand_3: Option<Token>
}

impl AssemblerInstruction {
    /// Represents an Opcode instruction in terms of assembly.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => match code {
                _ => {
                    results.push(code as  u8);
                }
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand_1, &self.operand_2, &self.operand_3] {
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut results),
                None => {}
            }
        }

        return results;
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
    
    /// Extracts a series of bytes representing an operand and adds
    /// the results to a vector.
    match t {
        Token::Register { reg_num } => {
            results.push(*reg_num);
        }
        Token::IntegerOperand { value } => {
            let converted = *value as u16;
            let byte1 = converted;
            let byte2 = converted >> 8;
            results.push(byte2 as u8);
            results.push(byte1 as u8);
        }
        _ => {
            println!("Opcode found in operand field");
            std::process::exit(1);
        }
    }        

    }

}

/// Parses instructions of the form LOAD $0 #100.
named!(pub instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        op: opcode_load >>
        reg: register >>
        integer_op: integer_operand >>
        (
            AssemblerInstruction {
                opcode: op,
                operand_1: Some(reg),
                operand_2: Some(integer_op),
                operand_3: None
            }
        )
    )

);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_one() {
        let result = instruction_one(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD},
                    operand_1: Some(Token::Register { reg_num: 0}),
                    operand_2: Some(Token::Number { value:  100}),
                    operand_3: None
                }
            ))
        )
    }
}