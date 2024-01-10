pub(crate) mod num;

#[derive(Debug)]
pub(crate) enum LexerToken {
    //    Identifier(String),
    //    Keyword(String),
    Number(isize),
}

pub(crate) fn lex(buffer: &str) -> Vec<LexerToken> {
    let result = vec![];
    let mut iterator = buffer.chars().peekable();

    while let Some(&char) = iterator.peek() {
        if char.is_ascii_digit() {
            num::lex_number(&mut iterator); // this is just here so cargo check does not complain about unused code :sob:
        }
    }

    result
}
