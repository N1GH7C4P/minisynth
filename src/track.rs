use crate::WavetableOscillator;
use rodio::{OutputStreamHandle, source::Source, Sink};
use std::time::Duration;

pub struct Track {
	sink: Sink,
	oscillator: WavetableOscillator,
	notes: Vec<f32>
}

impl Track {
	pub fn new(stream_handle: &OutputStreamHandle) -> Track {
	    let track = Track {
		sink: Sink::try_new(&stream_handle).unwrap(),
		oscillator: WavetableOscillator::preset_sin(44100),
		notes: Vec::new()
	    };
	    track
	}

	pub fn add_note(&mut self, frequency: f32) {
		self.notes.push(frequency);
	}

	pub fn emit(&mut self, tempo: u32) {
		let beat = 60.0 / tempo as f32;
		for n in 0..self.notes.len() {
			let mut tmp = self.oscillator.clone();
			tmp.set_frequency(self.notes[n]);
			self.sink.append(tmp.take_duration(Duration::from_secs_f32(beat)));
		}
	}
	
	pub fn sleep_until_end(&mut self)
	{
		self.sink.sleep_until_end();
	}
}
