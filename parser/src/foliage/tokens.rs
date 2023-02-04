use super::Tag;

#[derive(Debug, PartialEq)]
pub enum Token {
    Command,
    DocumentRoot,
    Environment,
    Literal,
    Paragraph
}

impl Tag for Token {
    fn value(&self) -> &str {
        match *self {
            Token::Command => "cmd",
            Token::DocumentRoot => "root",
            Token::Environment => "env",
            Token::Literal => "literal",
            Token::Paragraph => "paragraph",
        }
    }
}