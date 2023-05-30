use crate::expression::Expr;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
}
