use arborist::TokenTree;
use symbolize::{Symbol, SymbolTable};
use dbg_pls::DebugPls;

#[derive(Clone, Debug, DebugPls)]
pub enum Value {
    Atom(Symbol),
    Variable(Symbol),
    Functor(Functor),
}

#[derive(Clone, Debug, DebugPls)]
pub struct Functor {
    name: Symbol,
    args: Vec<Value>,
}

#[derive(Clone, Debug, DebugPls)]
pub enum Clause {
    Fact(Functor),
    Rule(Functor, Value),
}

pub fn parse(_tt: &[TokenTree], _symbol_table: &mut SymbolTable) -> Vec<Clause> {
    todo!()
}
