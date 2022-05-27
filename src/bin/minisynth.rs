use rodio::OutputStream;
use minisynth::track::Track;
use minisynth::parser::parser;
use minisynth::keymap::Keymap;
use std::env;
use std::io::{Error, ErrorKind};
use std::fs::{self};
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::{thread, time};

fn stream(mut tracks: Vec<Track>) {
	for t in 0..tracks.len() {
	    tracks[t].load_sink();
	}
	for t in 0..tracks.len() {
	    tracks[t].play();
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
	let keymap = Keymap::new("assets/notes_freq_lower.json");
	let desc: Vec<String> = parser::get_desc(&args[1]);
	let tempo = parser::get_tempo(&desc);
	let mut tracks = parser::get_tracks(&desc, &stream_handle);
	parser::set_notes(&mut tracks, &desc, tempo, keymap.get());
	
	let mut contents: Vec<String> = Vec::new();
	let filenames: Vec<PathBuf> = match fs::read_dir("../../assets/ascii") {
        Err(e) if e.kind() == ErrorKind::NotFound => Vec::new(),
        Err(e) => panic!("Unexpected Error! {:?}", e),
        Ok(entries) => entries.filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect()
    };

	let nb_files: usize = filenames.len();

	for n in 0..nb_files
	{
		let mut f = File::open(&filenames[n]).expect("file not found");
		f.read_to_string(&mut contents[n])
			.expect("something went wrong reading the file");
	}
	thread::spawn(move || {
		let frame_time = time::Duration::from_millis(200);
		loop{
			for n in 0..nb_files
			{
				println!("\n{}", contents[n]);
				thread::sleep(frame_time);
			}
		}
    });
	println!("And begin!");
	stream(tracks);	
}
