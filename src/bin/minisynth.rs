use rodio::{OutputStream};
use minisynth::track::Track;
use std::env;
use std::fs;

fn stream(mut tracks: Vec<Track>, tempo: u32) {
	for t in 0..tracks.len() {
	    tracks[t].emit(tempo);
	}
	for t in 0..tracks.len() {
	    tracks[t].sleep_until_end();
	}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
	println!("Usage: ./minisynth file");
	std::process::exit(1);
    }
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let tempo = 160;
    let mut tracks : Vec<Track> = Vec::new();
    tracks.push(Track::new(&stream_handle));
    tracks.push(Track::new(&stream_handle));
    tracks[0].add_note(261.63);
    tracks[0].add_note(293.66);
    tracks[0].add_note(329.63);
    tracks[0].add_note(349.23);
    tracks[0].add_note(329.63);
    tracks[0].add_note(293.66);
    tracks[1].add_note(65.42);
    tracks[1].add_note(65.42);
    tracks[1].add_note(65.42);
    tracks[1].add_note(200.42);
    tracks[1].add_note(200.42);
    tracks[1].add_note(200.42);
    stream(tracks, tempo);
}
