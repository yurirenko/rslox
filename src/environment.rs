use crate::expression::LiteralValue;
use crate::token::Token;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, v: LiteralValue) {
        self.values.insert(name.to_string(), v);
    }

    pub fn assign(&mut self, name: &str, v: LiteralValue) {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), v);
        } else {
            panic!("Variable {} is not defined", name);
        }
    }

    pub fn get(&mut self, token: &Token) -> LiteralValue {
        self.values
            .get(&token.lexeme)
            .unwrap_or_else(|| panic!("Variable {} is not defined", token.lexeme))
            .clone()
    }
}
