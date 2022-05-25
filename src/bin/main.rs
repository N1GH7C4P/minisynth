use rodio::{OutputStream, source::Source};
use minisynth::WavetableOscillator;

fn main() {
    let mut oscillator = WavetableOscillator::preset_saw(44100);
    oscillator.set_frequency(523.25);
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    let _result = stream_handle.play_raw(oscillator.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(5));
}
