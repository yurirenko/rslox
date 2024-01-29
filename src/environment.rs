use crate::expression::LiteralValue;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: &str, v: LiteralValue) {
        self.values.insert(name.to_string(), v);
    }

    pub fn assign(&mut self, name: &str, v: LiteralValue) {
        match self.values.contains_key(name) {
            true => {
                self.values.insert(name.to_string(), v);
            }
            false => {
                if let Some(enclosing_env) = &mut self.enclosing {
                    enclosing_env.assign(name, v);
                } else {
                    panic!("Variable {} is not defined", name);
                }
            }
        }
    }

    pub fn get(&self, name: &str) -> LiteralValue {
        match self.values.get(name) {
            Some(value) => value.clone(),
            None => {
                if let Some(enclosed_env) = &self.enclosing {
                    enclosed_env.get(name)
                } else {
                    panic!("Variable {} is not defined", name)
                }
            }
        }
    }
}

#[test]
fn test_define() {
    let number_val = 3.14;
    let expected_value = LiteralValue::Number(number_val);
    let mut env = Environment::new(None);

    env.define("pi", expected_value.clone());
    assert_eq!(env.get("pi"), expected_value);
}

#[test]
fn test_define_enclosed() {
    let expected_value = LiteralValue::Number(3.14);
    let mut enclosed_env = Box::new(Environment::new(None));
    enclosed_env.define("pi", expected_value.clone());

    let env = Environment::new(Some(enclosed_env));
    assert_eq!(env.get("pi"), expected_value);
}

#[test]
fn test_define_enclosed_no_value() {
    let expected_value = LiteralValue::Number(3.14);
    let enclosed_env = Box::new(Environment::new(None));

    let mut env = Environment::new(Some(enclosed_env));

    env.define("pi", expected_value.clone());
    assert_eq!(env.get("pi"), expected_value);

    let enclosed_env = env.enclosing.unwrap();

    let enclosed_env_value = std::panic::catch_unwind(|| enclosed_env.get("pi"));
    assert!(enclosed_env_value.is_err());
}

#[test]
fn test_define_enclosed_with_shadowing() {
    let shadowed_value = LiteralValue::Number(3.14);
    let mut enclosed_env = Box::new(Environment::new(None));

    enclosed_env.define("pi", shadowed_value.clone());

    let new_value = LiteralValue::Number(10.0);

    let mut env = Environment::new(Some(enclosed_env));
    env.define("pi", new_value.clone());
    assert_eq!(env.get("pi"), new_value);

    let enclosed_env = env.enclosing.unwrap();
    assert_eq!(enclosed_env.get("pi"), shadowed_value);
}

#[test]
fn test_assign() {
    let number_val = 3.147;
    let expected_value = LiteralValue::Number(number_val);

    let mut env = Environment::new(None);

    env.define("pi", LiteralValue::Number(3.14));
    env.assign("pi", expected_value.clone());

    assert_eq!(env.get("pi"), expected_value);
}

#[test]
#[should_panic]
fn test_assign_no_value() {
    let number_val = 3.147;
    let expected_value = LiteralValue::Number(number_val);

    let mut env = Environment::new(None);

    env.define("pi", expected_value);
    env.assign("tau", LiteralValue::Number(6.28));
}

#[test]
fn test_assign_enclosed() {
    let shadowed_value = LiteralValue::Number(3.14);
    let mut enclosed_env = Box::new(Environment::new(None));

    enclosed_env.define("pi", shadowed_value.clone());

    let new_value = LiteralValue::Number(10.0);

    let mut env = Environment::new(Some(enclosed_env));
    env.assign("pi", new_value.clone());

    assert_eq!(env.get("pi"), new_value);
}

#[test]
fn test_assign_enclosed_with_shadowing() {
    let shadowed_value = LiteralValue::Number(3.14);
    let mut enclosed_env = Box::new(Environment::new(None));

    enclosed_env.define("pi", shadowed_value.clone());

    let new_value = LiteralValue::Number(10.0);

    let mut env = Environment::new(Some(enclosed_env));
    env.define("pi", shadowed_value.clone());
    env.assign("pi", new_value.clone());

    assert_eq!(env.get("pi"), new_value);
}
