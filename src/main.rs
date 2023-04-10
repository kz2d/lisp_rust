use std::io::{stdin, self, Write};

use crate::{repl::repl_exec, env::Env};

mod lexer;
mod vars;
mod env;
mod parser;
mod runer;
mod repl;
mod tools;

fn main() {
    println!("Hello, world!");
    // lexer("sad");
    print!(">>> ");
    io::stdout().flush().unwrap();
    let mut env = Env::new();
    for i in stdin().lines() {
        repl_exec(i.unwrap(), &mut env);
        print!(">>> ");
        io::stdout().flush().unwrap();
    }
}
