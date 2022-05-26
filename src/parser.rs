pub mod parser {
	use std::fs;
	use rodio::OutputStreamHandle;
	use crate::track::Track;
	use regex::Regex;

	pub fn get_desc(filename: &String) -> Vec<String> {
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

	pub fn get_tempo(desc: &Vec<String>) -> u32 {
		let tempo_str: Vec<&str> = desc[0].as_str().split(' ').collect();
		tempo_str[1].trim().parse::<u32>().unwrap()
	}

	pub fn get_tracks(desc: &Vec<String>, stream_handle: &OutputStreamHandle) -> Vec<Track> {
		let tmp_arr: Vec<&str> = desc[1].as_str().split(' ').collect();
		let instruments: Vec<&str> = tmp_arr[1].split(',').collect();
		let mut tracks : Vec<Track> = Vec::new();
		for instrument in instruments.iter() {
			tracks.push(Track::new(&stream_handle, &instrument));
		}
		tracks
	}

	fn get_note_arr(line: &String) -> Vec<String> {
		let line_start = line.find(':').unwrap() + 1;
		let mut trimmed_line = line.get(line_start..).unwrap().replace("|","").trim().to_string();
		let re = Regex::new(r"\s+").unwrap();
		trimmed_line = re.replace_all(trimmed_line.as_str(), " ").to_string();
		trimmed_line.as_str().split(' ').map(|s| s.to_string()).collect()
	}

	pub fn set_notes(tracks: &mut Vec<Track>, desc: &Vec<String>, tempo: u32) {
		let mut track_num: u32;
		for (index, line) in desc.iter().enumerate() {
			if index < 2 {
				continue;
			}
			track_num = line.chars().nth(0).unwrap().to_digit(10).unwrap() - 1;
			let note_arr: Vec<String> = get_note_arr(line);
			tracks[track_num as usize].add_notes(note_arr, tempo);
		}
	}
}
