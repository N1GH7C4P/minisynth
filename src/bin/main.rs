use std::time::Duration;
use rodio::{OutputStream, OutputStreamHandle, source::Source, Sink};
use minisynth::WavetableOscillator;

struct Track {
    sink: Sink,
    oscillator: WavetableOscillator,
    notes: Vec<f32>
}

fn emit(sink: &Sink, oscillator: &WavetableOscillator, frequency: f32, seconds: f32) {
    let mut tmp = oscillator.clone();
    tmp.set_frequency(frequency);
    sink.append(tmp.take_duration(Duration::from_secs_f32(seconds)));
}

fn new_track(stream_handle: &OutputStreamHandle) -> Track {
    let track = Track {
        sink: Sink::try_new(&stream_handle).unwrap(),
        oscillator: WavetableOscillator::preset_sin(44100),
        notes: Vec::new()
    };
    track
}

fn stream(tracks: Vec<Track>, tempo: u32, length: usize) {
    let beat = 60.0 / tempo as f32;
    for n in 0..length {
        for t in 0..tracks.len() {
	    emit(&tracks[t].sink, &tracks[t].oscillator, tracks[t].notes[n], beat);
        }
    }
    for t in 0..tracks.len() {
        tracks[t].sink.sleep_until_end();
    }
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let tempo = 160;
    let mut tracks : Vec<Track> = Vec::new();
    tracks.push(new_track(&stream_handle));
    tracks.push(new_track(&stream_handle));
    tracks[0].notes.push(261.63);
    tracks[0].notes.push(293.66);
    tracks[0].notes.push(329.63);
    tracks[0].notes.push(349.23);
    tracks[0].notes.push(329.63);
    tracks[0].notes.push(293.66);
    tracks[1].notes.push(65.42);
    tracks[1].notes.push(65.42);
    tracks[1].notes.push(65.42);
    tracks[1].notes.push(73.42);
    tracks[1].notes.push(73.42);
    tracks[1].notes.push(73.42);
    stream(tracks, tempo, 6);
}
