use crate::toketype::*;

pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: String,
    line: isize,
}


impl Token {
    pub fn new(
        ttype: TokenType, lexeme: String,
        literal: String, line: isize
    ) -> Self {
        Token {
            ttype, lexeme, literal, line
        }   
    }

    fn to_string(&self) -> String {
        format!("{:?} {} {}", self.ttype, self.lexeme, self.literal)
    }
}
