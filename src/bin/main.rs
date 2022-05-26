use std::time::Duration;
use rodio::{OutputStream, source::Source, Sink};
use minisynth::WavetableOscillator;

fn emit(sink: &Sink, oscillator: &WavetableOscillator, frequency: f32, seconds: f32) {
    let mut tmp = oscillator.clone();
    tmp.set_frequency(frequency);
    sink.append(tmp.take_duration(Duration::from_secs_f32(seconds)));
}
fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sine = WavetableOscillator::preset_sin(44100);
    let sink = Sink::try_new(&stream_handle).unwrap();
    emit(&sink, &sine, 440.00, 0.25);
    emit(&sink, &sine, 523.25, 0.25);
    sink.sleep_until_end();
}
