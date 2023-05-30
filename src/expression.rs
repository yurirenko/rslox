use crate::statement::Statement;
use crate::token::Token;
#[cfg(test)]
use crate::token::TokenType;
use colored::Colorize;
#[cfg(test)]
use pretty_assertions::assert_eq;
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
    fn visit_binary_expression(&self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_grouping_expression(&self, expr: &Expr) -> R;
    fn visit_literal_expression(&self, value: &LiteralValue) -> R;
    fn visit_unary_expression(&self, operator: &Token, expr: &Expr) -> R;
    fn visit_expression(&self, expr: &Expr) -> R;
    fn visit_variable_expression(&self, expr: &Expr) -> R;

    fn visit_statement(&self, statement: &Statement);
    fn visit_var_statement(&self, statement: &Statement);
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        AstPrinter.visit_expression(expr)
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut parenthesized_expr = String::new();
        parenthesized_expr.push('(');
        parenthesized_expr.push_str(&name);
        for e in exprs {
            parenthesized_expr.push(' ');
            let accept_result = self.print(e);

            parenthesized_expr.push_str(&accept_result)
        }
        parenthesized_expr.push(')');

        parenthesized_expr
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expression(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.as_str(), vec![left, right])
    }

    fn visit_grouping_expression(&self, expr: &Expr) -> String {
        self.parenthesize("group", vec![expr])
    }

    fn visit_literal_expression(&self, value: &LiteralValue) -> String {
        match value {
            LiteralValue::Nil => String::from("nil"),
            LiteralValue::String(string_value) => String::from(string_value),
            LiteralValue::Number(number_value) => number_value.to_string(),
            LiteralValue::Boolean(boolean_value) => boolean_value.to_string(),
        }
    }

    fn visit_unary_expression(&self, operator: &Token, expr: &Expr) -> String {
        self.parenthesize(operator.lexeme.as_str(), vec![expr])
    }

    fn visit_variable_expression(&self, expr: &Expr) -> String {
        self.parenthesize("var", vec![expr])
    }

    fn visit_var_statement(&self, statement: &Statement) {
        todo!()
    }

    fn visit_expression(&self, e: &Expr) -> String {
        match e {
            Expr::Binary(left, operator, right) => {
                self.visit_binary_expression(left, operator, right)
            }
            Expr::Unary(operator, expr) => self.visit_unary_expression(operator, expr),
            Expr::Grouping(expr) => self.visit_grouping_expression(expr),
            Expr::Literal(value) => self.visit_literal_expression(value),
            Expr::Variable(_) => self.visit_variable_expression(e),
        }
    }

    fn visit_statement(&self, statement: &Statement) {
        match statement {
            Statement::Expression(expr) => {
                self.visit_expression(expr);
            }
            Statement::Print(expr) => {
                self.parenthesize("print", vec![expr]);
            },
            _ => {
                todo!();
            }
        }
    }
}

#[test]
fn test_ast_printer() {
    let expr = Expr::Binary(
        Box::new(Expr::Unary(
            Token {
                token_type: TokenType::Minus,
                lexeme: String::from("-"),
                line: 1,
            },
            Box::new(Expr::Literal(LiteralValue::Number(123.into()))),
        )),
        Token {
            token_type: TokenType::Star,
            lexeme: String::from("*"),
            line: 1,
        },
        Box::new(Expr::Grouping(Box::new(Expr::Literal(
            LiteralValue::Number(45.68),
        )))),
    );

    let printer = AstPrinter {};
    let actual = printer.print(&expr);

    assert_eq!("(* (- 123) (group 45.68))", actual);
}
