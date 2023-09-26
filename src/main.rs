#![feature(if_let_guard)]

use std::io::{self, stdin, Write};

use crate::{env::Env, repl::repl_exec};

mod env;
mod lexer;
mod parser;
mod repl;
mod runer;
mod std_lisp;
mod tools;
mod vars;

fn main() {
    print!(">>> ");
    io::stdout().flush().unwrap();
    let mut env = Env::new();
    for i in stdin().lines() {
        repl_exec(i.unwrap(), &mut env);
        print!(">>> ");
        io::stdout().flush().unwrap();
    }
}
