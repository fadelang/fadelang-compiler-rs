use crate::compiler::lexer::parentheses::BracketKind;

pub(crate) mod comments;
pub(crate) mod identifiers;
pub(crate) mod number;
pub(crate) mod operators;
pub(crate) mod parentheses;
pub(crate) mod whitespace;

macro_rules! lex {
    { source = $input:ident, lexers = $($lexer:ident)+ } => {
        let mut result = vec![];
        let mut iterator = $input.chars().peekable();

        while let Some(_) = iterator.peek() {
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
    /* Comments and Whitespace */
    Whitespace,
    Comment(String),

    /* Identifiers and Keywords */
    Identifier(String),
    Keyword(String),

    /* Literals */
    Number(isize),
    FloatingPointNumber(f64),

    Character(char),
    Boolean(bool),
    String(String),

    /* Punctuation */
    Parentheses(BracketKind),
    Operator(String), // todo: replace String with custom enum
}

pub(crate) fn lex(input: &str) -> Vec<LexerToken> {
    lex! {
        source = input,
        lexers = comments identifiers number operators parentheses whitespace
    }
}
