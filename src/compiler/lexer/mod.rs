use super::keywords::Keyword;
use crate::compiler::lexer::parentheses::Parentheses;

pub(crate) mod identifiers;
pub(crate) mod number;
pub(crate) mod parentheses;

#[derive(Debug)]
pub(crate) enum LexerToken {
    Identifier(String),
    Keyword(Keyword),
    Parentheses(Parentheses),
    Number(isize),
}

pub(crate) fn lex(input: &str) -> Vec<LexerToken> {
    let mut result = vec![];
    let mut iterator = input.chars().peekable();

    while let Some(peek) = iterator.peek() {
        let mut none = true;

        if let Some(token) = identifiers::lex_identifier_or_keyword(&mut iterator) {
            result.push(token);
            none = false;
        }

        if let Some(token) = number::lex_number(&mut iterator) {
            result.push(token);
            none = false;
        }

        if let Some(token) = parentheses::lex_parentheses(&mut iterator) {
            result.push(token);
            none = false;
        }

        if none {
            break;
        }
    }

    result
}
