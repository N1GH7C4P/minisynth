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
		if desc[index].len() < 2  {
			println!("Empty line! {}", index);
			desc.remove(index);
			continue;
		}
		else if desc[index].chars().nth(0).unwrap() == '#' {
			desc.remove(index);
		}
		/*
		if desc[index].chars().nth(0) > Some('0') &&  desc[index].chars().nth(0) <= Some('9'){
			println!("{}", desc[index].as_str.unwrap());
			continue;
		}
		*/
	}
	desc
}


fn get_notes(desc: &Vec<String>, line: u32){
	let notes_str: Vec<&str> = desc[line as usize].as_str().split(' ').collect();
	for index in (0..notes_str.len()) {
		//println!("{}", notes_str[index]);
		let note_parts: Vec<&str> = notes_str[index].split('/').collect();
		if(note_parts.len() == 1)
		{
			println!("f: {}", note_parts[0]);
		}
		else
		{
			println!("f: {} t: {}", note_parts[0], note_parts[1]);
		}

	}

	//let notes: Vec<&str> = 
	//let times: Vec<&f32> = 
}

fn get_tempo(desc: &Vec<String>) -> u32 {
	let tempo_str: Vec<&str> = desc[0].as_str().split(' ').collect();
	//println!("tempo_str: {}", tempo_str[0]);
	let tempo: u32 = tempo_str[1].trim().parse::<u32>().unwrap();
	return tempo;
}

fn set_tracks(tracks: &mut Vec<Track>, desc: &Vec<String>, stream_handle: &OutputStreamHandle) {
	let tmp_arr: Vec<&str> = desc[1].as_str().split(' ').collect();
	let instruments: Vec<&str> = tmp_arr[1].split(',').collect();
	for instrument in instruments.iter() {
		//println!("{}", &instrument);
		tracks.push(Track::new(&stream_handle, &instrument));
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
	print!("{}", tempo);

	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	//println!("len {}", tracks.len());
	set_tracks(&mut tracks, &desc, &stream_handle);
	//println!("len {}", tracks.len());
	get_notes(&desc, 4);
//	stream(tracks, tempo);
}
