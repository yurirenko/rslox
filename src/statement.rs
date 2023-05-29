use crate::expression::Expr;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expr),
    Print(Expr),
}
