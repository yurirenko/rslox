use crate::expression::LiteralValue;
use crate::token::Token;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Option<LiteralValue>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: &str, v: Option<LiteralValue>) {
        println!("set {:?} to {:?}", name, v);
        self.values.insert("hey".into(), Some(LiteralValue::Number(42.0)));
        self.values.insert(name.to_string(), v);
        println!("hash map {:?}", self.values);
    }

    pub fn get(&mut self, token: &Token) -> LiteralValue {
        println!("get {:?}", &token.lexeme);
        // self.values.insert(token.lexeme.clone(), Some(LiteralValue::Number(42.0)));

        println!("hash map {:?}", self.values);
        self.values.get(&token.lexeme)
            .as_ref()
            .unwrap_or_else(|| panic!("Variable {} is not defined", token.lexeme))
            .as_ref()
            .unwrap_or_else(|| panic!("Variable {} is not initialized", token.lexeme))
            .clone()
    }
}
