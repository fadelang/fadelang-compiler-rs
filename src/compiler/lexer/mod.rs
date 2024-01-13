use super::keywords::Keyword;
use crate::compiler::lexer::parentheses::Parentheses;

pub(crate) mod identifiers;
pub(crate) mod number;
pub(crate) mod operators;
pub(crate) mod parentheses;
pub(crate) mod whitespace;

macro_rules! lex {
    { source = $input:ident, lexers = $($lexer:ident)+ } => {
        let mut result = vec![];
        let mut iterator = $input.chars().peekable();

        while let Some(_peek) = iterator.peek() {
            let mut none = true;

            $(if let Some(token) = $lexer::lex(&mut iterator) {
                result.push(token);
                none = false;
            })+

            if none { break; }
        }
        result
    }
}

#[derive(Debug)]
pub(crate) enum LexerToken {
    Identifier(String),
    Keyword(Keyword),
    Parentheses(Parentheses),
    Number(isize),
    Whitespace,
}

pub(crate) fn lex(input: &str) -> Vec<LexerToken> {
    lex! {
        source = input,
        lexers = identifiers number operators parentheses whitespace
    }
}
