#[derive(Debug)]
pub(crate) enum Keyword {
    Use,
}

pub(crate) fn get_keyword(string: &str) -> Option<Keyword> {
    match string {
        "use" => Some(Keyword::Use),
        _ => None,
    }
}
