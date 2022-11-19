use chumsky::prelude::*;
use chumsky::text::whitespace;
use dbg_pls::DebugPls;
use std::cell::RefCell;
use symbolize::{Symbol, SymbolTable};

#[derive(Clone, Copy, Debug, DebugPls)]
pub enum Token {
    Identifier(Symbol),
    Atom(Symbol),
    Comma,
    DogsBollocks,
    Period,
    ParenClose,
    ParenOpen,
}

pub fn lex(source: &str, symbol_table: &mut SymbolTable) -> Vec<Token> {
    //let table_refcell = RefCell::new(symbol_table);
    token(&RefCell::new(symbol_table))
        .then_ignore(whitespace())
        .repeated()
        .parse(source.trim_start())
        .unwrap()
}

fn token<'a: 'b, 'b>(
    symbol_table: &'a RefCell<&'b mut SymbolTable>,
) -> impl Parser<char, Token, Error = Simple<char>> + 'a {
    let comma = just(',').to(Token::Comma);
    let period = just('.').to(Token::Period);
    let paren_open = just('(').to(Token::ParenOpen);
    let paren_close = just(')').to(Token::ParenClose);
    let balls = just(":-").to(Token::DogsBollocks);
    let symbol = filter(|&c: &char| c.is_alphanumeric() || c == '_' || c == '-' || c == '\'')
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|x| symbol_table.borrow_mut().create_symbol(x));
    let ident = symbol.map(Token::Identifier);
    let atom = just('`').ignore_then(symbol).map(Token::Atom);
    choice((comma, period, paren_open, paren_close, balls, atom, ident))
}
