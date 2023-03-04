use super::Tag;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Command,
    DocumentRoot,
    Environment,
    Literal,
    Options,
    Paragraph
}

impl Tag for Token {
    fn value(&self) -> &str {
        match *self {
            Token::Command => "cmd",
            Token::DocumentRoot => "root",
            Token::Environment => "env",
            Token::Literal => "literal",
            Token::Options => "options",
            Token::Paragraph => "paragraph",
        }
    }

    fn is_front(&self) -> bool {
        false
    }
}