use crate::expression::{Expr, LiteralValue};
use crate::token::{Token, TokenType};
use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    pub tokens: Peekable<Iter<'a, Token>>,
    prev_token: Option<&'a Token>,
}

impl<'a> Parser<'a> {
    pub fn init(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
            prev_token: None,
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while let Some(token) = self.tokens.peek() {
            match token.token_type {
                TokenType::BangEqual | TokenType::EqualEqual => {
                    self.advance();

                    let op = self.prev_token.unwrap();
                    let right = self.comparison();

                    expr = Expr::Binary(Box::new(expr), op.clone(), Box::new(right));
                }
                _ => break,
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while let Some(token) = self.tokens.peek() {
            match token.token_type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    self.advance();

                    let op = self.prev_token.unwrap();
                    let right = self.term();

                    expr = Expr::Binary(Box::new(expr), op.clone(), Box::new(right));
                }
                _ => break,
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while let Some(token) = self.tokens.peek() {
            match token.token_type {
                TokenType::Minus | TokenType::Plus => {
                    self.advance();

                    let op = self.prev_token.unwrap();
                    let right = self.factor();

                    expr = Expr::Binary(Box::new(expr), op.clone(), Box::new(right));
                }
                _ => break,
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while let Some(token) = self.tokens.peek() {
            match token.token_type {
                TokenType::Minus | TokenType::Plus => {
                    self.advance();
                    let op = self.prev_token.unwrap();
                    let right = self.unary();

                    expr = Expr::Binary(Box::new(expr), op.clone(), Box::new(right));
                }
                _ => break,
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        match self.tokens.peek().unwrap().token_type {
            TokenType::Bang | TokenType::Minus => {
                self.advance();

                let op = self.prev_token.unwrap();
                let right = self.unary();
                Expr::Unary(op.clone(), Box::new(right))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expr {
        match &self.tokens.peek().unwrap().token_type {
            TokenType::False => {
                self.advance();
                Expr::Literal(LiteralValue::Boolean(false))
            }
            TokenType::True => {
                self.advance();
                Expr::Literal(LiteralValue::Boolean(true))
            }
            TokenType::Nil => {
                self.advance();
                Expr::Literal(LiteralValue::Nil)
            }
            TokenType::Number(n) => {
                self.advance();
                Expr::Literal(LiteralValue::Number(*n))
            }
            TokenType::StringLiteral(s) => {
                self.advance();
                Expr::Literal(LiteralValue::String(s.clone()))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();

                if let Some(token) = self.tokens.peek() {
                    match token.token_type {
                        TokenType::RightParen => {
                            self.advance();
                            Expr::Grouping(Box::new(expr))
                        }
                        _ => {
                            panic!("Missing closing parenthesis!");
                        }
                    }
                } else {
                    panic!("Missing closing parenthesis!");
                }
            }
            _ => {
                panic!("Syntax error!");
            }
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        self.prev_token = self.tokens.next();

        self.prev_token
    }
}

#[test]
fn test_binary() {
    let tokens = vec![
        Token {
            line: 1,
            lexeme: String::from("2.0"),
            token_type: TokenType::Number(2.0),
        },
        Token {
            line: 1,
            lexeme: String::from("+"),
            token_type: TokenType::Plus,
        },
        Token {
            line: 1,
            lexeme: String::from("3.0"),
            token_type: TokenType::Number(3.0),
        },
    ];
    let expected = Expr::Binary(
        Box::new(Expr::Literal(LiteralValue::Number(2.0))),
        Token {
            line: 1,
            lexeme: String::from("+"),
            token_type: TokenType::Plus,
        },
        Box::new(Expr::Literal(LiteralValue::Number(3.0))),
    );

    let mut parser = Parser::init(&tokens[..]);
    assert_eq!(expected, parser.parse());
}

#[test]
fn test_unary() {
    let tokens = vec![
        Token {
            line: 1,
            lexeme: String::from("-"),
            token_type: TokenType::Minus,
        },
        Token {
            line: 1,
            lexeme: String::from("10.0"),
            token_type: TokenType::Number(10.0),
        },
    ];

    let expected = Expr::Unary(
        Token {
            line: 1,
            lexeme: String::from("-"),
            token_type: TokenType::Minus,
        },
        Box::new(Expr::Literal(LiteralValue::Number(10.0))),
    );

    let mut parser = Parser::init(&tokens[..]);
    assert_eq!(expected, parser.parse());
}

#[test]
fn test_nested_unary() {
    let tokens = vec![
        Token {
            line: 1,
            lexeme: String::from("-"),
            token_type: TokenType::Minus,
        },
        Token {
            line: 1,
            lexeme: String::from("-"),
            token_type: TokenType::Minus,
        },
        Token {
            line: 1,
            lexeme: String::from("10.0"),
            token_type: TokenType::Number(10.0),
        },
    ];

    let expected = Expr::Unary(
        Token {
            line: 1,
            lexeme: String::from("-"),
            token_type: TokenType::Minus,
        },
        Box::new(Expr::Unary(
            Token {
                line: 1,
                lexeme: String::from("-"),
                token_type: TokenType::Minus,
            },
            Box::new(Expr::Literal(LiteralValue::Number(10.0))),
        )),
    );

    let mut parser = Parser::init(&tokens[..]);
    assert_eq!(expected, parser.parse());
}
