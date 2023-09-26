use crate::{env::Env, parser::Value};
use anyhow::Result;
use std::{fmt::Debug, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub input: Vec<String>,
    pub function: Box<Value>,
}

pub struct NativeFanction(pub Rc<dyn Fn(&mut Env, &Vec<&Value>) -> Result<Value>>);

impl Debug for NativeFanction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NativeFunction")
    }
}

impl PartialEq for NativeFanction {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref() as *const _ == other.0.as_ref() as *const _
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Clone for NativeFanction {
    fn clone(&self) -> Self {
        NativeFanction(Rc::clone(&self.0))
    }
}
