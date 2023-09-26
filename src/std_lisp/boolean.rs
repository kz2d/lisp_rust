use crate::add_operator;
use crate::cast;
use crate::parser::Value;
use crate::parser::Value::Boolean;
use crate::runer::exec;
use crate::vars::function::NativeFanction;
use anyhow::Result;
use std::rc::Rc;

use crate::env::Env;

pub fn register(context: &mut Env) -> Result<()> {
    add_operator!(context, "not", 1; Boolean => Boolean; |f: bool| !f);
    // TODO: yep
    Ok(())
}
