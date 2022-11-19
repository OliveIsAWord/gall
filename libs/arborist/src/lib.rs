use dbg_pls::DebugPls;
use lexer::Token;
use symbolize::Symbol;

#[derive(Clone, Debug, DebugPls)]
pub enum TokenTree {
    Parens(Vec<Self>),
    Identifier(Symbol),
    Atom(Symbol),
    Comma,
    Period,
    DogsBollocks,
}

#[must_use]
pub fn cultivate(tokens: Vec<Token>) -> Vec<TokenTree> {
    let mut trees = vec![vec![]];
    for token in tokens {
        let tree = match token {
            Token::ParenOpen => {
                trees.push(vec![]);
                continue;
            }
            Token::ParenClose => {
                let inner = trees.pop().expect("extra closing paren");
                TokenTree::Parens(inner)
            }
            Token::Comma => TokenTree::Comma,
            Token::Period => TokenTree::Period,
            Token::DogsBollocks => TokenTree::DogsBollocks,
            Token::Atom(s) => TokenTree::Atom(s),
            Token::Identifier(s) => TokenTree::Identifier(s),
        };
        trees.last_mut().unwrap().push(tree);
    }
    assert_eq!(trees.len(), 1, "unclosed paren");
    trees.pop().unwrap()
}
