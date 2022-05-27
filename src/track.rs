use crate::WavetableOscillator;
use rodio::{OutputStreamHandle, source::Source, Sink};
use std::time::Duration;
use std::collections::HashMap;

pub struct Track {
	sink: Sink,
	oscillator: WavetableOscillator,
	notes: Vec<(f32, f32)>,
	octave: u32,
	duration: f32
}

impl Track {
	pub fn new(stream_handle: &OutputStreamHandle, instrument: &str) -> Track {
	    let oscillator = if instrument.eq("sine") {
		WavetableOscillator::preset_sin(44100)
	    }
	    else if instrument.eq("saw") {
		WavetableOscillator::preset_saw(44100)
	    }
	    else if instrument.eq("square") {
		WavetableOscillator::preset_sqr(44100)
	    }
	    else if instrument.eq("triangle") {
		WavetableOscillator::preset_tri(44100)
	    }
	    else {
		WavetableOscillator::preset_sin(44100)
	    };

	    let track = Track {
		sink: Sink::try_new(&stream_handle).unwrap(),
		oscillator: oscillator,
		notes: Vec::new(),
		octave: 4,
		duration: 1.0
	    };
	    track
	}

	fn add_note(&mut self, frequency: f32, duration: f32) {
		self.notes.push((frequency, duration));
	}

	fn update_duration(&mut self, note: &String) -> String {
		let index = note.find('/').unwrap() + 1;
		self.duration = note.get(index..).unwrap().parse::<f32>().unwrap();
		let test: Vec<&str> = note.as_str().split('/').collect();
		test[0].to_string()
	}

	fn update_note(&mut self, note: &String) -> String {
		if note.len() == 1 {
			if note.chars().nth(0).unwrap() == 'r' {
				return note.to_string();
			}
			let key = format!("{}{}", note, self.octave);
			return key;
		}
		if note.len() == 2 && !note.chars().nth(1).unwrap().is_digit(10) {
			let key = format!("{}{}", note, self.octave);
			return key;
		}
		if note.len() == 2 && note.chars().nth(1).unwrap().is_digit(10) {
			self.octave = note.chars().nth(1).unwrap().to_digit(10).unwrap();
			let key = format!("{}", note);
			return key;
		}
		self.octave = note.chars().nth(2).unwrap().to_digit(10).unwrap();
		let key = format!("{}", note);
		return key
	}
	
	pub fn add_notes(&mut self, notes: Vec<String>, tempo: u32, key_freq: &HashMap<String, f32>) {
		let beat: f32 = 60.0 / tempo as f32;
		for note in notes.iter() {
			if note.find('/').is_some() {
				let key: &String = &self.update_duration(note);
				let res = self.update_note(key);
				self.add_note(key_freq[&res], beat * self.duration);
				continue;
			}
			let res = self.update_note(note);
			self.add_note(key_freq[&res], beat * self.duration);
		}
	}

	pub fn emit(&mut self) {
		for n in 0..self.notes.len() {
			let mut tmp = self.oscillator.clone();
			tmp.set_frequency(self.notes[n].0);
			self.sink.append(tmp.take_duration(Duration::from_secs_f32(self.notes[n].1)));
		}
	}
	
	pub fn sleep_until_end(&mut self)
	{
		self.sink.sleep_until_end();
	}
}
