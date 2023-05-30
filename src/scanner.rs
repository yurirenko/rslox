use crate::token::{Token, TokenType};
#[cfg(test)]
use pretty_assertions::assert_eq;
use std::collections::VecDeque;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    line: u32,
}

#[derive(Debug)]
enum TokenizerError {
    UnterminatedString,
}

impl<'a> Scanner<'a> {
    pub fn init(source: &'a str) -> Self {
        Scanner {
            // enumerate instead?
            // if we do, we can have index of the current character
            source: source.chars().peekable(),
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = self.source.next() {
            match c {
                '(' => tokens.push(self.make_token(c.to_string(), TokenType::LeftParen)),
                ')' => tokens.push(self.make_token(c.to_string(), TokenType::RightParen)),
                '{' => tokens.push(self.make_token(c.to_string(), TokenType::LeftBrace)),
                '}' => tokens.push(self.make_token(c.to_string(), TokenType::RightBrace)),
                ',' => tokens.push(self.make_token(c.to_string(), TokenType::Comma)),
                '.' => tokens.push(self.make_token(c.to_string(), TokenType::Period)),
                '-' => tokens.push(self.make_token(c.to_string(), TokenType::Minus)),
                '+' => tokens.push(self.make_token(c.to_string(), TokenType::Plus)),
                '*' => tokens.push(self.make_token(c.to_string(), TokenType::Star)),
                ';' => tokens.push(self.make_token(c.to_string(), TokenType::Semicolon)),
                '!' => {
                    let mut op = TokenType::Bang;
                    let mut lexeme = c.to_string();
                    if let Some('=') = self.source.peek() {
                        op = TokenType::BangEqual;
                        lexeme.push_str("=");
                        self.source.next();
                    }
                    tokens.push(self.make_token(lexeme, op));
                }
                '=' => {
                    let mut op = TokenType::Equal;
                    let mut lexeme = c.to_string();
                    if let Some('=') = self.source.peek() {
                        op = TokenType::EqualEqual;
                        lexeme.push_str("=");
                        self.source.next();
                    }
                    tokens.push(self.make_token(lexeme, op));
                }
                '<' => {
                    let mut op = TokenType::Less;
                    let mut lexeme = c.to_string();
                    if let Some('=') = self.source.peek() {
                        op = TokenType::LessEqual;
                        lexeme.push_str("=");
                        self.source.next();
                    }
                    tokens.push(self.make_token(lexeme, op));
                }
                '>' => {
                    let mut op = TokenType::Greater;
                    let mut lexeme = c.to_string();
                    if let Some('=') = self.source.peek() {
                        op = TokenType::GreaterEqual;
                        lexeme.push_str("=");
                        self.source.next();
                    }
                    tokens.push(self.make_token(lexeme, op));
                }
                '/' => {
                    if let Some('/') = self.source.peek() {
                        while self.source.peek().map_or(false, |&c| c != '\n') {
                            self.source.next();
                        }
                    } else if let Some('*') = self.source.peek() {
                        while let Some(c) = self.source.next() {
                            match c {
                                '*' => {
                                    if let Some('/') = self.source.peek() {
                                        self.source.next();
                                        break;
                                    }
                                }
                                '\n' => {
                                    self.line += 1;
                                }
                                _ => {}
                            }
                        }
                    } else {
                        tokens.push(self.make_token(c.to_string(), TokenType::Slash));
                    }
                }
                ' ' | '\r' | '\t' => (),
                '\n' => {
                    self.line += 1;
                }
                '"' => {
                    tokens.push(self.tokenize_string_literal().unwrap());
                }
                '0'..='9' => {
                    tokens.append(self.tokenize_number(c).as_mut());
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    tokens.push(self.tokenize_identifier(c));
                }
                _ => {
                    eprintln!("Unexpected character {} at line {}", c, self.line);
                }
            }
        }

        tokens.push(self.make_token(String::from(""), TokenType::Eof));

        tokens
    }

    fn make_token(&self, lexeme: String, token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme,
            line: self.line,
        }
    }

    fn tokenize_string_literal(&mut self) -> Result<Token, TokenizerError> {
        let mut string_literal = String::from("");

        while self.source.peek().map_or(false, |&c| c != '"') {
            if let Some(char) = self.source.peek() {
                if char == &'\n' {
                    self.line += 1;
                }

                string_literal.push(*char);
                self.source.next();
            }
        }

        if self.source.peek().is_none() {
            eprintln!("Unterminated string at {}", self.line);
            Err(TokenizerError::UnterminatedString)
        } else {
            self.source.next();
            Ok(Token {
                token_type: TokenType::StringLiteral(string_literal.to_string()),
                lexeme: string_literal,
                line: self.line,
            })
        }
    }

    fn tokenize_identifier(&mut self, char: char) -> Token {
        let mut token = char.to_string();

        while let Some(char) = self.source.peek() {
            if char.is_ascii_alphanumeric() || *char == '_' {
                token.push(*char);
                self.source.next();
            } else {
                break;
            }
        }

        self.make_token(
            token.clone(),
            TokenType::make_keyword(token.as_str()).unwrap_or(TokenType::Identifier(token)),
        )
    }

    fn tokenize_number(&mut self, c: char) -> Vec<Token> {
        let mut tokens: VecDeque<Token> = VecDeque::new();
        let current_line = self.line;
        let mut number = c.to_string();

        while let Some('0'..='9') = self.source.peek() {
            match self.source.next() {
                Some(c) => number.push(c),
                None => break,
            }
        }
        if let Some(c) = self.source.peek() {
            if c == &'.' {
                // consume the period
                // since we can't peek twice, we need to consume the period and either
                // make a token for it (if the next character after the period is not a number)
                // or add the period as part of the number
                self.source.next();

                if let Some('0'..='9') = self.source.peek() {
                    number.push('.');
                } else {
                    let token = Token {
                        lexeme: ".".to_string(),
                        line: current_line,
                        token_type: TokenType::Period,
                    };
                    tokens.push_back(token);
                }
            }
        }

        while let Some('0'..='9') = self.source.peek() {
            match self.source.next() {
                Some(c) => number.push(c),
                None => break,
            }
        }

        tokens.push_front(self.make_token(
            number.clone(),
            TokenType::Number(number.parse::<f64>().unwrap()),
        ));
        tokens.into()
    }
}

#[test]
fn test_scan_tokens() {
    let input = "<>(){   }\"hello!\"
    // and a comment!
    !=
    24)
    102.56
    var string_val = 102.to_string()
    /* multi-line
    comment goes here !!!
    and then ends here? */ var test;
    +
    ";

    let expected_output = vec![
        Token {
            line: 1,
            lexeme: String::from("<"),
            token_type: TokenType::Less,
        },
        Token {
            line: 1,
            lexeme: String::from(">"),
            token_type: TokenType::Greater,
        },
        Token {
            line: 1,
            lexeme: String::from("("),
            token_type: TokenType::LeftParen,
        },
        Token {
            line: 1,
            lexeme: String::from(")"),
            token_type: TokenType::RightParen,
        },
        Token {
            line: 1,
            lexeme: String::from("{"),
            token_type: TokenType::LeftBrace,
        },
        Token {
            line: 1,
            lexeme: String::from("}"),
            token_type: TokenType::RightBrace,
        },
        Token {
            line: 1,
            lexeme: String::from("hello!"),
            token_type: TokenType::StringLiteral(String::from("hello!")),
        },
        Token {
            line: 3,
            lexeme: String::from("!="),
            token_type: TokenType::BangEqual,
        },
        Token {
            line: 4,
            lexeme: String::from("24"),
            token_type: TokenType::Number(24.0),
        },
        Token {
            line: 4,
            lexeme: String::from(")"),
            token_type: TokenType::RightParen,
        },
        Token {
            line: 5,
            lexeme: String::from("102.56"),
            token_type: TokenType::Number(102.56),
        },
        Token {
            line: 6,
            lexeme: String::from("var"),
            token_type: TokenType::Var,
        },
        Token {
            line: 6,
            lexeme: String::from("string_val"),
            token_type: TokenType::Identifier(String::from("string_val")),
        },
        Token {
            line: 6,
            lexeme: String::from("="),
            token_type: TokenType::Equal,
        },
        Token {
            line: 6,
            lexeme: String::from("102"),
            token_type: TokenType::Number(102.0),
        },
        Token {
            line: 6,
            lexeme: String::from("."),
            token_type: TokenType::Period,
        },
        Token {
            line: 6,
            lexeme: String::from("to_string"),
            token_type: TokenType::Identifier(String::from("to_string")),
        },
        Token {
            line: 6,
            lexeme: String::from("("),
            token_type: TokenType::LeftParen,
        },
        Token {
            line: 6,
            lexeme: String::from(")"),
            token_type: TokenType::RightParen,
        },
        Token {
            line: 9,
            lexeme: String::from("var"),
            token_type: TokenType::Var,
        },
        Token {
            line: 9,
            lexeme: String::from("test"),
            token_type: TokenType::Identifier(String::from("test")),
        },
        Token {
            line: 9,
            lexeme: String::from(";"),
            token_type: TokenType::Semicolon,
        },
        Token {
            line: 10,
            lexeme: String::from("+"),
            token_type: TokenType::Plus,
        },
        Token {
            line: 11,
            lexeme: String::from(""),
            token_type: TokenType::Eof,
        },
    ];

    let mut scanner = Scanner::init(input);
    let actual_output = scanner.scan_tokens();

    assert_eq!(expected_output, *actual_output);
}

#[test]
#[should_panic(expected = "UnterminatedString")]
fn test_scan_tokens_with_unterminated_string() {
    let input = "<>(){   }\"hello!\
    \
    // and a comment!
    !=   \
    23";

    let mut scanner = Scanner::init(input);
    scanner.scan_tokens();
}
