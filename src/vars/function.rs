use crate::parser::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
	pub input: Vec<String>,
	pub function: Box<Value>
}