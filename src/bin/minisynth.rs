use rodio::{OutputStream, OutputStreamHandle};
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

fn get_desc(filename: &String) -> Vec<String> {
	let contents = fs::read_to_string(&filename).expect("Something went wrong reading the file");
	let mut desc: Vec<String> = contents.as_str().split('\n').map(str::to_string).collect();
	for index in (0..desc.len()).rev() {
		if desc[index].len() == 0 {
			desc.remove(index);
			continue;
		}
		if desc[index].chars().nth(0).unwrap() == '#' {
			desc.remove(index);
		}
	}
	desc
}

fn get_tempo(desc: &Vec<String>) -> u32 {
	let tempo_str: Vec<&str> = desc[0].as_str().split(' ').collect();
	tempo_str[1].parse().unwrap()
}

fn set_tracks(tracks: &mut Vec<Track>, desc: &Vec<String>, stream_handle: &OutputStreamHandle) {
	let tmp_arr: Vec<&str> = desc[1].as_str().split(' ').collect();
	let instruments: Vec<&str> = tmp_arr[1].split(',').collect();
	for instrument in instruments.iter() {
		tracks.push(Track::new(&stream_handle));
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2
	{
	    println!("Usage: ./minisynth file");
	    std::process::exit(1);
	}
	let mut tracks : Vec<Track> = Vec::new();
	let desc: Vec<String> = get_desc(&args[1]);
	let tempo = get_tempo(&desc);

	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	println!("len {}", tracks.len());
	set_tracks(&mut tracks, &desc, &stream_handle);
	println!("len {}", tracks.len());
//	stream(tracks, tempo);
}
