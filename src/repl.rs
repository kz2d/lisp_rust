use crate::{lexer::lexer, parser::parse, runer::run, env::Env};

pub fn repl_exec(input: String, env: &mut Env) {
	for i in run(parse(lexer(&input)), env) {
		println!("{i}");
	}
}