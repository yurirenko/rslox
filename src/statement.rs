use crate::expression::Expr;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expr),
    Print(Expr),
    Block(Vec<Statement>),
    Var(Token, Option<Expr>),
}
