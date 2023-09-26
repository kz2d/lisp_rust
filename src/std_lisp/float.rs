use crate::add_operator;
use std::rc::Rc;
use anyhow::Result;
use crate::cast;
use crate::parser::Value::{Float, Boolean};
use crate::parser::Value;
use crate::runer::exec;
use crate::vars::function::NativeFanction;

use crate::env::Env;

pub fn register(context: &mut Env) -> Result<()> {
    add_operator!(context, "+", 2; Float, Float => Float; |f,s| f+s);
    add_operator!(context, "-", 2; Float, Float => Float; |f,s| f-s);
    add_operator!(context, "*", 2; Float, Float => Float; |f,s| f*s);
    add_operator!(context, "/", 2; Float, Float => Float; |f,s| f/s);
    add_operator!(context, "%", 2; Float, Float => Float; |f,s| f%s);
    add_operator!(context, "//", 2; Float, Float => Float; |f,s| if f/s >0.0 {((f/s) as f64).floor()}else{((f/s) as f64).ceil()});
    add_operator!(context, "=", 2; Float, Float => Boolean; |f,s| f==s);
    add_operator!(context, ">", 2; Float, Float => Boolean; |f,s| f>s);
    add_operator!(context, "<", 2; Float, Float => Boolean; |f,s| f<s);
    
    Ok(())
}


#[macro_export]
macro_rules! add_operator {
    ($context:tt, $name: tt, $input_len:tt; $( $input:path),* => $output:path; $function: expr ) => {
    $context.set_global(
        $name.to_string(),
        Value::NativeFunction(NativeFanction(Rc::new(|env, inp| {
            anyhow::ensure!(inp.len() == $input_len, "number of arguments is wrong for function {}", $name);
            let mut i:i32 = -1;
            Ok($output(
               $function( 
                $({i+=1;
                cast!(exec(&inp[i as usize], env)?, $input)?}),*)
            ))
        }))),
    );
    };
}
