use std::{clone, slice::Iter};

pub fn lexer<'a>(program: &'a String) -> Vec<String> {
    let mut word = (0, 0);
    let mut out: Vec<String> = Vec::new();
    for (index, i) in program.chars().enumerate() {
        match i {
            '(' | ')' => {
                if word.0 != word.1 {
                    out.push(program[word.0..word.1].into());
                    word.0 = word.1;
                }

                out.push(program[index..index + 1].into());
            }
            ' ' | '\t' | '\n' => {
                if word.0 != word.1 {
                    out.push(program[word.0..word.1].into());
                    word.0 = word.1;
                }
            }
            x if x.is_alphabetic() || x.is_numeric() || x.is_ascii() => {
                if word.0 == word.1 {
                    word = (index, index);
                }
                word.1 += 1;
            }
            _ => {
                panic!("fuck wrong char in lexer");
            }
        }
    }

    out
}

#[cfg(test)]
mod test {
    use crate::lexer::lexer;

    #[test]
    fn test_lexer_first() {
        assert_eq!(
            lexer(&"(add 1 2)".to_string()),
            vec!["(", "add", "1", "2", ")"]
        );
    }

    #[test]
    fn test_lexer_random() {
        assert_eq!(
            lexer(
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
            ),
            vec![
                "(",
                "define",
                "sum",
                "(",
                "lambda",
                "(",
                "x",
                "y",
                ")",
                "(",
                "+",
                "x",
                "y",
                ")",
                ")",
                ")",
                "(",
                "sum",
                "10",
                "20.0",
                ")",
                "(",
                "define",
                "sqr",
                "(",
                "lambda",
                "(",
                "x",
                ")",
                "(",
                "*",
                "x",
                "x",
                ")",
                ")",
                ")",
                "(",
                "sqr",
                "20",
                ")",
                "(",
                "define",
                "area-of-circle",
                "(",
                "lambda",
                "(",
                "r",
                ")",
                "(",
                "*",
                "pi",
                "(",
                "sqr",
                "r",
                ")",
                ")",
                ")",
                ")",
                "(",
                "area-of-circle",
                "10.0",
                ")",
                "(",
                "define",
                "is-even",
                "(",
                "lambda",
                "(",
                "x",
                ")",
                "(",
                "=",
                "0",
                "(",
                "%",
                "x",
                "2",
                ")",
                ")",
                ")",
                ")",
                "(",
                "is-even",
                "10",
                ")"
            ]
        );
    }
}
