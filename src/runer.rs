use crate::vars::function::Function;
use crate::{parser::Value, cast};
use crate::env::Env;

pub fn run(ast: Vec<Value>, env: &mut Env) -> Vec<String> {
	let mut out = Vec::new();
	for i in ast {
		out.push(format!("{:?}", exec(&i, env)));
	}
	out
}


fn exec<'a>(v: &Value, env: &mut Env) -> Value {
	match v {
		Value::Scoup(sc) => {
			let name = cast!(&sc[0], Value::Name);
			let mut args = Vec::new();
			for i in 1..sc.len() {
				args.push(&sc[i]);
			}
			match name.as_str() {
				"+" => {
					assert_eq!(sc.len(), 3);
					Value::Float(cast!(exec(&args[0], env), Value::Float) + cast!(exec(&args[1], env), Value::Float))
				}
				"-" if sc.len() == 3 => {
					assert_eq!(sc.len(), 3);
					Value::Float(cast!(exec(&args[0], env), Value::Float) - cast!(exec(&args[1], env), Value::Float))
				}
				"*" => {
					assert_eq!(sc.len(), 3);
					Value::Float(cast!(exec(&args[0], env), Value::Float) * cast!(exec(&args[1], env), Value::Float))
				}
				"=" => {
					assert_eq!(sc.len(), 3);
					Value::Boolean(cast!(exec(&args[0], env), Value::Float) == cast!(exec(&args[1], env), Value::Float))
				}
				">" => {
					assert_eq!(sc.len(), 3);
					Value::Boolean(cast!(exec(&args[0], env), Value::Float) > cast!(exec(&args[1], env), Value::Float))
				}
				"<" => {
					assert_eq!(sc.len(), 3);
					Value::Boolean(cast!(exec(&args[0], env), Value::Float) < cast!(exec(&args[1], env), Value::Float))
				}
				"not" => {
					assert_eq!(sc.len(), 2);
					Value::Boolean(!cast!(exec(&args[0], env), Value::Boolean))
				}
				"if" => {
					assert_eq!(sc.len(), 4);
					let v = cast!(exec(&args[0], env), Value::Boolean);
					if v {
						exec(&args[1], env)
					}else{
						exec(&args[2], env)
					}
				}
				"lambda" => {
					let vars: Vec<String> = cast!(args[0], Value::Scoup).iter().map(|x| cast!(x, Value::Name).to_string()).collect();
					Value::Function(Function{input: vars, function: Box::new(args[1].to_owned())})
				}
				"let" => {
					let vars = cast!(args[0], Value::Scoup);
					env.create_scoup();
					if if let Value::Scoup(_) = &vars[0] {true} else {false} {
						for i in vars {
							let var = cast!(i, Value::Scoup);
							let val = exec(&var[1], env);
							env.set(cast!(var[0].clone(), Value::Name), val);
						}
					}else{
						let var = vars;
						let val = exec(&var[1], env);
						env.set(cast!(var[0].clone(), Value::Name), val);
					}
					let out = exec(args[1], env);

					env.drop_scoup();
					out
				}
				x if env.get(x.to_string()).is_some() => {
					let fun = cast!(env.get(x.to_string()).unwrap(), Value::Function);
					env.create_scoup();
					for (name, val) in fun.input.iter().zip(args) {
						let val = exec(val, env);
						env.set(name.to_owned(), val);
					}
					let out = exec(&fun.function, env);

					env.drop_scoup();

					out
				}
				_ => {
					panic!("not implemented function {}", name);
				}
			}
		}
		Value::Name(x) if env.get(x.to_string()).is_some() => {
			exec(&env.get(x.to_string()).unwrap(), env)
		}
		x => {
			x.clone()
		}
	}
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::lexer,
		env::Env,
        parser::{
            parse,
            Value::{Float, Name, Scoup},
        }, runer::run,
    };

    #[test]
    fn test_runer_first() {
        assert_eq!(
            run(parse(lexer(&"(+ 1 2)".to_string())), &mut Env::new()),
            vec!["Float(3.0)"]
        );
		assert_eq!(
            run(parse(lexer(&"(+ 1 (+ 4 2))".to_string())), &mut Env::new()),
            vec!["Float(7.0)"]
        );
    }

	#[test]
    fn test_runer_fibonachi() {
        assert_eq!(
            run(parse(lexer(&"(let (
					fib (lambda (x)
							(if (< x 3) 
								1 
								(+ (fib (- x 1)) (fib (- x 2)))
							)
					)
				) (fib 20))".to_string())), &mut Env::new()),
            vec!["Float(6765.0)"]
        );
    }
}