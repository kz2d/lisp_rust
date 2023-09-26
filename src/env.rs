use std::{collections::HashMap,};
use crate::parser::Value;
#[derive(Clone)]
pub struct Node {
	vars: HashMap<String, Value>,
	prev_env: Option<Box<Node>>
}

#[derive(Clone)]
pub struct Env(Box<Node>);

impl Env {
	pub fn new() -> Env {
		Env(Box::new(Node{vars: HashMap::new(), prev_env: None}))
	}

	pub fn get(&self, name: String) -> Option<Value> {
		self.find_not_mut(name).cloned()
	}

	pub fn update(&mut self, name: String, val: Value) -> bool {
		let el = self.find(name);
		if el.is_none() {
			return false;
		}
		*(el.unwrap()) = val;
		true
	}

	pub fn set(&mut self, name: String, val: Value) -> bool {
		self.0.vars.insert(name, val).is_none()
	}

	pub fn set_global(&mut self, name: String, val: Value) -> bool {

		let mut p = self.0.as_mut();
		while p.prev_env.is_some() {
			let j = p.prev_env.as_mut().unwrap().as_mut();
			p = j;
		}
		p.vars.insert(name, val).is_none()
	}

	pub fn create_scoup(&mut self) {
		let n = Some(self.0.clone());
		*self = Env::new();
		self.0.prev_env = n;
	}

	pub fn drop_scoup(&mut self) {
		let n = &self.0;
		self.0 = n.prev_env.as_ref().unwrap().to_owned();
	}

	fn find<'a>(&'a mut self, name: String) -> Option<&'a mut Value> {
		let mut h = self.0.vars.get_mut(&name);
		let mut e = self.0.prev_env.as_mut();

		while h.is_none() && e.is_some() {
			let j = e.unwrap();
			h = j.vars.get_mut(&name);
			e = j.prev_env.as_mut();
		}

		h
	}

	fn find_not_mut<'a>(&'a self, name: String) -> Option<&'a Value> {
		let mut h = self.0.vars.get(&name);
		let mut e = self.0.prev_env.as_ref();

		while h.is_none() && e.is_some() {
			let j = e.unwrap();
			h = j.vars.get(&name);
			e = j.prev_env.as_ref();
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
		let env2 = &mut Env::new();
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

	#[test]
	fn global_scoup() {
		let mut env = Env::new();

		env.create_scoup();
		env.create_scoup();
		env.create_scoup();

		env.set_global("x".to_owned(), Name("fdf".to_owned()));
		assert_eq!(env.get("x".to_string()), Some(Name("fdf".to_string())));

		env.set("x".to_string(), Float(2.3));

		assert_eq!(env.get("x".to_string()), Some(Float(2.3)));

		env.drop_scoup();
		env.drop_scoup();
		assert_eq!(env.get("x".to_string()), Some(Name("fdf".to_string())));

		env.drop_scoup();
		assert_eq!(env.get("x".to_string()), Some(Name("fdf".to_string())));
	}
}