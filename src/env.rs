use std::{collections::HashMap, ops::Deref};
use crate::parser::Value;
#[derive(Clone)]
pub struct Node {
	vars: HashMap<String, Value>,
	prevEnv: Option<Box<Node>>
}

#[derive(Clone)]
pub struct Env(Box<Node>);

impl Env {
	pub fn new() -> Env {
		Env(Box::new(Node{vars: HashMap::new(), prevEnv: None}))
	}

	pub fn get(&self, name: String) -> Option<Value> {
		self.find_not_mut(name).cloned()
	}

	pub fn update(&mut self, name: String, val: Value) -> bool {
		let mut el = self.find(name);
		if el.is_none() {
			return false;
		}
		*(el.unwrap()) = val;
		true
	}

	pub fn set(&mut self, name: String, val: Value) -> bool {
		self.0.vars.insert(name, val).is_none()
	}

	pub fn create_scoup(&mut self) {
		let mut n = Some(self.0.clone());
		self.0 = Env::new().0;
		self.0.prevEnv = n;
	}

	pub fn drop_scoup(&mut self) {
		let n = &self.0;
		self.0 = n.prevEnv.as_ref().unwrap().to_owned();
	}

	fn find<'a>(&'a mut self, name: String) -> Option<&'a mut Value> {
		let mut h = self.0.vars.get_mut(&name);
		let mut e = self.0.prevEnv.as_mut();

		while h.is_none() && e.is_some() {
			let j = e.unwrap();
			h = j.vars.get_mut(&name);
			e = j.prevEnv.as_mut();
		}

		h
	}

	fn find_not_mut<'a>(&'a self, name: String) -> Option<&'a Value> {
		let mut h = self.0.vars.get(&name);
		let mut e = self.0.prevEnv.as_ref();

		while h.is_none() && e.is_some() {
			let j = e.unwrap();
			h = j.vars.get(&name);
			e = j.prevEnv.as_ref();
		}

		h
	}
}

#[cfg(test)]
mod test {
    use super::Env;
	use crate::parser::Value::{Float, Name};

	#[test]
	fn env_first() {
		let mut env2 = &mut Env::new();
		env2.set("x".to_string(),  Float(2.2));

		assert_eq!(env2.get("x".to_string()), Some(Float(2.2)));

		env2.create_scoup();

		env2.update("x".to_string(), Float(3.0));
		env2.set("y".to_string(), Name("hoho".to_string()));
		env2.set("x".to_string(), Name("x".to_string()));


		assert_eq!(env2.get("x".to_string()), Some(Name("x".to_string())));
		assert_eq!(env2.get("y".to_string()), Some(Name("hoho".to_string())));

		env2.drop_scoup();

		assert_eq!(env2.get("x".to_string()), Some(Float(3.0)));
		assert_eq!(env2.get("y".to_string()), None);
	}
}