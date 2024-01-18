use crate::compiler::operators::Operator;

use super::LexerToken;
use std::iter::Peekable;

pub(crate) fn lex<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
where
    T: Iterator<Item = char>,
{
    let peek = *iterator.peek()?;

    let operator = match peek {
        '-' => {
            iterator.next()?;
            match *iterator.peek()? {
                '>' => {
                    iterator.next()?;
                    Some(Operator::ReturnTypeSpecifier)
                }
                _ => Some(Operator::Subtraction),
            }
        }
        ':' => {
            iterator.next()?;
            Some(Operator::TypeSpecifier)
        }
        ';' => {
            iterator.next()?;
            Some(Operator::StatementTerminator)
        }
        _ => None,
    };

    let operator = operator?;
    Some(LexerToken::Operator(operator))
}
