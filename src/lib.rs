#[macro_use]
pub mod expression;


#[allow(unused_imports)]
use expression::{Expression, BinOP};

#[derive(Clone, Debug, PartialEq)]
pub enum Program<'a> {
    FunctionDef {
        name: &'a str,
        args: Vec<&'a str>,
        body: Statement<'a>
    },
    Statement(Statement<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement<'a> {
    LetBind {
        name: &'a str,
        expr: Expression
    },
    Expression(Expression),
}
