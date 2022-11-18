#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Atom(String),
    Variable(String),
    Functor(Functor),
}

#[derive(Clone, Debug)]
pub struct Functor {
    name: String,
    args: Vec<Value>,
}

#[derive(Clone, Debug)]
pub enum Clause {
    Fact(Functor),
    Rule(Functor, Value),
}

impl Clause {
    pub fn head(&self) -> &Functor {
        match self {
            Self::Fact(f) | Self::Rule(f, _) => f,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Module {
    clauses: Vec<Clause>,
}

impl Module {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }
    pub fn eval(&self, f: Functor) -> bool {
        println!("evaling: {f:?}");
        let Functor { name, args } = f;
        'clause: for clause in &self.clauses {
            let head = clause.head();
            if head.name != name || head.args.len() != args.len() {
                continue;
            }
            println!("Found potential rule: {clause:?}");
            let mut var_map: HashMap<String, String> = HashMap::new();
            for z in head.args.iter().zip(&args) {
                use Value::*;
                match z {
                    (Atom(s1), Atom(s2)) => {
                        if s1 != s2 {
                            continue 'clause;
                        }
                    }
                    (Atom(a), Variable(v)) | (Variable(v), Atom(a)) => {
                        var_map.insert(v.to_owned(), a.to_owned());
                    }
                    e => todo!("{:?}", e),
                }
            }
            println!("var_map: {var_map:?}");
            match clause {
                Clause::Fact(_) => return true,
                Clause::Rule(_, v) => match v {
                    Value::Functor(f) => {
                        let name = f.name.clone();
                        let args = f.args.iter().cloned().map(|x| {
                            let Value::Variable(ref v) = x else { return x };
                            match var_map.get(v) {
                                Some(new_val) => Value::Atom(new_val.to_string()),
                                None => x,
                            }
                        }).collect();
                        return self.eval(Functor { name, args });
                    }
                    e => todo!("eval on non-functor {e:?}"),
                },
            }
        }
        false
    }
}

fn main() {
    classic();
}

fn classic() {
    let mut program = Module::new();
    program.add_clause(Clause::Fact(Functor {
        name: "man".to_owned(),
        args: vec![Value::Atom("Socrates".to_owned())],
    }));
    program.add_clause(Clause::Rule(
        Functor {
            name: "mortal".to_owned(),
            args: vec![Value::Variable("x".to_owned())],
        },
        Value::Functor(Functor {
            name: "man".to_owned(),
            args: vec![Value::Variable("x".to_owned())],
        }),
    ));
    let evaled = program.eval(Functor {
        name: "mortal".to_owned(),
        args: vec![Value::Atom("Socrates".to_owned())],
    });
    println!("{evaled}");
}
