use dbg_pls::DebugPls;
use indexmap::IndexSet;
use std::borrow::Cow;
use std::fmt;

#[derive(Clone, Copy, Debug, DebugPls)]
pub struct Symbol(u64);

impl Symbol {
    #[must_use]
    fn new(index: usize) -> Self {
        let index = u64::try_from(index).unwrap();
        Self(index)
    }
    #[must_use]
    pub fn index(self) -> usize {
        usize::try_from(self.0).unwrap()
    }
}

#[derive(Default)]
pub struct SymbolTable {
    map: IndexSet<String>,
}

impl SymbolTable {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    // TODO: Remove this annotation when `anonymous_lifetime_in_impl_trait` stabilizes
    #[must_use]
    pub fn create_symbol<'a>(&mut self, symbol: impl Into<Cow<'a, str>>) -> Symbol {
        // Forward to a non-generic function to reduce code bloat
        self.create_symbol_impl(symbol.into())
    }
    #[must_use]
    fn create_symbol_impl(&mut self, symbol: Cow<'_, str>) -> Symbol {
        let index = self
            .map
            .get_index_of(&*symbol)
            .unwrap_or_else(|| self.map.insert_full(symbol.into_owned()).0);
        Symbol::new(index)
    }
}

impl fmt::Debug for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SymbolTable{:?}", self.map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut table = SymbolTable::new();
        assert_eq!(table.create_symbol("meow").0, 0);
        assert_eq!(table.create_symbol("nya".to_owned()).0, 1);
        assert_eq!(table.create_symbol("meow".to_owned()).0, 0);
        assert_eq!(table.create_symbol("nya").0, 1);
    }
}
