use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::{named, tag, do_parse, types::CompleteStr};

/// opcode parser provides a simple piece of logic to load
/// various opcodes, to be parsed by nom.
named!(pub opcode_load<CompleteStr, Token>,

    do_parse!(
        tag!("load") >> (Token::Op{code: Opcode::LOAD})
    )
);

mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load(CompleteStr("load"));

        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op{code: Opcode::LOAD});
        assert_eq!(rest, CompleteStr(""));

        let result = opcode_load(CompleteStr("oald"));
        assert_eq!(result.is_ok(), false);
    }
}