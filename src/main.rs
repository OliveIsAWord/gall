use std::fs::read_to_string;
use symbolize::SymbolTable;

fn main() {
    let source = read_to_string("example.gall").unwrap();
    let mut symbol_table = SymbolTable::new();
    let tokens = lexer::lex(&source, &mut symbol_table);
    dbg!(&tokens);
}
