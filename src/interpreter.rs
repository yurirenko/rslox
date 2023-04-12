use crate::expression::{Expr, LiteralValue, Visitor};
use crate::token::{Token, TokenType};

pub struct Interpreter;

impl Visitor<LiteralValue> for Interpreter {
    fn visit_binary_expression(&self, left: &Expr, operator: &Token, right: &Expr) -> LiteralValue {
        let left_right = (self.visit_expression(left), self.visit_expression(right));

        match left_right {
            (LiteralValue::Boolean(left), LiteralValue::Boolean(right)) => {
                match operator.token_type {
                    TokenType::EqualEqual => LiteralValue::Boolean(left == right),
                    TokenType::BangEqual => LiteralValue::Boolean(left != right),
                    _ => unimplemented!("Unsupported operation for booleans")
                }
            },
            (LiteralValue::Number(left), LiteralValue::Number(right)) => {
                match operator.token_type {
                    TokenType::EqualEqual => LiteralValue::Boolean(left == right),
                    TokenType::BangEqual => LiteralValue::Boolean(left != right),
                    TokenType::Plus => LiteralValue::Number(left + right),
                    TokenType::Minus => LiteralValue::Number(left - right),
                    TokenType::Slash => LiteralValue::Number(left / right),
                    TokenType::Star => LiteralValue::Number(left * right),
                    TokenType::Greater => LiteralValue::Boolean(left > right),
                    TokenType::GreaterEqual => LiteralValue::Boolean(left >= right),
                    TokenType::Less => LiteralValue::Boolean(left < right),
                    TokenType::LessEqual => LiteralValue::Boolean(left <= right),
                    _ => unimplemented!("Unsupported operation for two numbers")
                }
            },
            (LiteralValue::String(left), LiteralValue::String(right)) => {
                match operator.token_type {
                    TokenType::Plus => {
                        let mut new_string = left;
                        new_string.push_str(&right);
                        LiteralValue::String(new_string)
                    },
                    _  => unimplemented!("Unsupported operation for two strings")
                }
            }
            _ => {
                unimplemented!("Operands have different types!");
            }
        }
    }

    fn visit_grouping_expression(&self, expr: &Expr) -> LiteralValue {
        self.visit_expression(expr)
    }

    fn visit_literal_expression(&self, value: &LiteralValue) -> LiteralValue {
        value.clone()
    }

    fn visit_unary_expression(&self, operator: &Token, expr: &Expr) -> LiteralValue {
        let right = self.visit_expression(expr);

        match right {
            LiteralValue::Boolean(boolean_value) => {
                if operator.token_type == TokenType::Bang {
                    LiteralValue::Boolean(!boolean_value)
                } else {
                    unimplemented!("Only negation operator is supported for booleans");
                }
            }
            LiteralValue::Nil => {
                unimplemented!("Unary operator cannot be applied to \"nil\"");
            }
            LiteralValue::Number(number) => {
                if operator.token_type == TokenType::Minus {
                    LiteralValue::Number(-number)
                } else {
                    unimplemented!("Only negation operator is supported for numbers");
                }
            }
            LiteralValue::String(_) => {
                unimplemented!("Unary operator cannot be applied to \"String\"");
            }
        }
    }

    fn visit_expression(&self, expr: &Expr) -> LiteralValue {
        match expr {
            Expr::Binary(left, operator, right) => {
                self.visit_binary_expression(left, operator, right)
            }
            Expr::Grouping(expr) => {
                self.visit_grouping_expression(expr)
            }
            Expr::Literal(literal_value) => {
                self.visit_literal_expression(literal_value)
            }
            Expr::Unary(operator, expr) => {
                self.visit_unary_expression(operator, expr)
            }
        }
    }
}