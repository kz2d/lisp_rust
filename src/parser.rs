use crate::vars::function::Function;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Float(f64),
    Name(String),
    Scoup(Vec<Value>),
	Function(Function),
    Boolean(bool)
}

pub fn parse(program: Vec<String>) -> Vec<Value> {
    let mut out = Vec::new();
    let mut stack = Vec::new();

    for i in program {
        match i.as_str() {
            x if x.parse::<f64>().is_ok() => {
                if let Value::Scoup(v) = stack.last_mut().unwrap() {
                    v.push(Value::Float(x.parse().unwrap()));
                }
            }
            "#f" => {
                if let Value::Scoup(v) = stack.last_mut().unwrap() {
                    v.push(Value::Boolean(false));
                }
            }
            "#t" => {
                if let Value::Scoup(v) = stack.last_mut().unwrap() {
                    v.push(Value::Boolean(true));
                }
            }
            "(" => {
                stack.push(Value::Scoup(Vec::new()));
            }
            ")" => {
                let val = stack.pop().expect("sorry too many )");
                if let Some(Value::Scoup(v)) = stack.last_mut() {
                    v.push(val);
                } else {
                    out.push(val);
                }
            }
            name => {
                if let Value::Scoup(v) = stack.last_mut().unwrap() {
                    v.push(Value::Name(name.to_string()));
                }
            }
        }
    }

	if !stack.is_empty() {
		panic!("too many open braces");
	}

    out
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::lexer,
        parser::{
            parse,
            Value::{Float, Name, Scoup},
        },
    };

    #[test]
    fn test_lexer_first() {
        assert_eq!(
            parse(lexer(&"(add 1 2)".to_string())),
            vec![Scoup(vec![Name("add".to_string()), Float(1.0), Float(2.0)])]
        );
    }

    #[test]
    fn test_lexer_random() {
        assert_eq!(
            parse(lexer(
                &"(define sum 
			(lambda (x y) (+ x y)))



		     (sum 10 20.0)
				
			(define sqr 
			(lambda (x) (* x x)))
			(sqr 20)

			(define area-of-circle 
			(lambda (r) (* pi (sqr r))))

			(area-of-circle 10.0)

			(define is-even 
			(lambda (x) (= 0 (% x 2))))

			(is-even 10)
			"
                .to_string()
            )),
            vec![
                Scoup(vec![
                    Name("define".to_string()),
                    Name("sum".to_string()),
                    Scoup(vec![
                        Name("lambda".to_string()),
                        Scoup(vec![Name("x".to_string()), Name("y".to_string())]),
                        Scoup(vec![Name("+".to_string()), Name("x".to_string()), Name("y".to_string())])
                    ])
                ]),
                Scoup(vec![Name("sum".to_string()), Float(10.0), Float(20.0)]),
                Scoup(vec![
                    Name("define".to_string()),
                    Name("sqr".to_string()),
                    Scoup(vec![
                        Name("lambda".to_string()),
                        Scoup(vec![Name("x".to_string())]),
                        Scoup(vec![Name("*".to_string()), Name("x".to_string()), Name("x".to_string())])
                    ])
                ]),
                Scoup(vec![Name("sqr".to_string()), Float(20.0)]),
                Scoup(vec![
                    Name("define".to_string()),
                    Name("area-of-circle".to_string()),
                    Scoup(vec![
                        Name("lambda".to_string()),
                        Scoup(vec![Name("r".to_string())]),
                        Scoup(vec![Name("*".to_string()), Name("pi".to_string()), Scoup(vec![Name("sqr".to_string()), Name("r".to_string())])])
                    ])
                ]),
                Scoup(vec![Name("area-of-circle".to_string()), Float(10.0)]),
                Scoup(vec![
                    Name("define".to_string()),
                    Name("is-even".to_string()),
                    Scoup(vec![
                        Name("lambda".to_string()),
                        Scoup(vec![Name("x".to_string())]),
                        Scoup(vec![
                            Name("=".to_string()),
                            Float(0.0),
                            Scoup(vec![Name("%".to_string()), Name("x".to_string()), Float(2.0)])
                        ])
                    ])
                ]),
                Scoup(vec![Name("is-even".to_string()), Float(10.0)])
            ]
        );
    }
}
