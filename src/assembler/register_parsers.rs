#[macro_use]
use crate::assembler::Token;
use nom::{named, ws, tag, digit, types::CompleteStr};

named!(pub register <CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: digit >>
            (
                Token::Register {
                    reg_num: reg_num.parse::<u8>().unwrap()
                }
            )
            
        )
    )
);

mod tests {
    use super::register;
    use nom::types::CompleteStr;
    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        let result = register(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);
        let result = register(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);
    }
}