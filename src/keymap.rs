use std::fs;
use serde_json::{Value};
use std::collections::HashMap;

pub struct Keymap {
	key_freq: HashMap<String, f32>
}

impl Keymap {
	pub fn new(filename: &str) -> Keymap {
		let data = fs::read_to_string(&filename).expect("Something went wrong reading the file");
		let map: Value = serde_json::from_str(data.as_str()).unwrap();
		let mut keymap = Keymap {
			key_freq: HashMap::new()
		};
		for entry in map.as_object().unwrap().iter() {
			keymap.key_freq.insert(entry.0.to_string(), entry.1.to_string().parse::<f32>().unwrap());
		}
		keymap
	}
	
	pub fn get(&self) -> &HashMap<String, f32> {
		&self.key_freq
	}
}
