use std::collections::HashMap;

use crate::parser::types::Type;

pub mod expressions;
pub mod statements;
pub mod types;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeError {
    Invalid(Type),
    Unexpected { got: Type, expected: Type },
    InvalidIdentifier(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    types: HashMap<String, Type>,
    vars: HashMap<String, Type>,
}

impl Scope {
    pub fn get_type(&self, name: &str) -> Option<Type> {
        self.types.get(name).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_type(name))
        })
    }
    pub fn set_type(&mut self, name: &str, ty: Type) {
        self.types.insert(name.to_string(), ty);
    }
    pub fn get_var(&self, name: &str) -> Option<Type> {
        self.vars
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.get_var(name)))
    }
    pub fn set_var(&mut self, name: &str, ty: Type) {
        self.vars.insert(name.to_string(), ty);
    }
    pub fn new() -> Self {
        Self {
            parent: None,
            types: HashMap::new(),
            vars: HashMap::new(),
        }
    }
    fn create_child(&self) -> Self {
        Self {
            parent: Some(Box::new(self.clone())),
            vars: HashMap::new(),
            types: HashMap::new(),
        }
    }
}

pub fn expect_type(ty: Type, expected: Type) -> Result<Type, TypeError> {
    if ty == expected {
        Ok(ty)
    } else {
        Err(TypeError::Unexpected { got: ty, expected })
    }
}
