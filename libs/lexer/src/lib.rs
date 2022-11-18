use symbolize::{Symbol, SymbolTable};

#[derive(Debug)]
pub enum Token {
    Identifier(Symbol),
    Atom(Symbol),
    ParenClose,
    ParenOpen,
    DogsBollocks,
    Period,
    Comma,
}

pub fn lex(source: &str, symbol_table: &mut SymbolTable) -> Vec<Token> {
    let mut tokens = vec![];
    let mut buffer = source.trim_start();
    while !buffer.is_empty() {
        let (token, rest) = token(buffer, symbol_table).unwrap();
        tokens.push(token);
        buffer = rest.trim_start();
    }
    tokens
}

pub fn token<'a>(_source: &'a str, _symbol_table: &mut SymbolTable) -> Option<(Token, &'a str)> {
    let _comma = parse_char(',', || Token::Comma);
    let _period = parse_char('.', || Token::Period);
    let _bollocks = parse_str(":-", || Token::DogsBollocks);
    todo!()
}

fn parse_char(c: char, mut t: impl FnMut() -> Token) -> impl FnMut(&str) -> Option<(Token, &str)> {
    move |src| (src.chars().next()? == c).then(|| (t(), &src[c.len_utf8()..]))
}

fn parse_str<'a>(
    s: &'a str,
    mut t: impl FnMut() -> Token + 'a,
) -> impl FnMut(&str) -> Option<(Token, &str)> + 'a {
    move |src| (src.starts_with(s)).then(|| (t(), &src[s.len()..]))
}

fn repeat(
    mut f: impl FnMut(&str) -> Option<&str>,
    mut finally: impl FnMut(&str) -> Option<Token>,
) -> impl FnMut(&str) -> Option<(Token, &str)> + 'a {
    |mut s| { let mut rest = None; while let Some(new_rest) = f(s) { rest = new_rest; s = new_rest; } }
}
