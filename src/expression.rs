use crate::statement::Statement;
use crate::token::Token;
use colored::Colorize;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Boolean(value) => {
                write!(f, "{}", value.to_string().blue())
            }
            LiteralValue::Nil => {
                write!(f, "{}", "nil".red())
            }
            LiteralValue::Number(value) => {
                write!(f, "{}", value.to_string().yellow())
            }
            LiteralValue::String(value) => {
                write!(f, "\"{}\"", value.to_string().green())
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(LiteralValue),
    Unary(Token, Box<Expr>),
    // for accessing the variable, not defining it!
    Variable(Token),
}

pub trait Visitor<R> {
    fn visit_binary_expression(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_grouping_expression(&mut self, expr: &Expr) -> R;
    fn visit_literal_expression(&mut self, value: &LiteralValue) -> R;
    fn visit_unary_expression(&mut self, operator: &Token, expr: &Expr) -> R;
    fn visit_expression(&mut self, expr: &Expr) -> R;
    fn visit_variable_expression(&mut self, name_token: &Token) -> R;

    fn visit_statement(&mut self, statement: &Statement);
    fn visit_var_declaration_statement(&mut self, token: &Token, initializer: &Option<Expr>);
}
