use core::time::Duration;
use rodio::source::Source;
pub mod track;
pub mod parser;
pub mod keymap;

const WAVE_TABLE_SIZE: usize = 64;

#[derive(Clone)]
pub struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }

    pub fn preset_sin(sample_rate: u32) -> WavetableOscillator {
        let mut wave_table: Vec<f32> = Vec::with_capacity(WAVE_TABLE_SIZE);
        for n in 0..WAVE_TABLE_SIZE {
            wave_table.push((2.0 * std::f32::consts::PI * n as f32 / WAVE_TABLE_SIZE as f32).sin());
        }
        return WavetableOscillator::new(sample_rate, wave_table);
    }

    pub fn preset_saw(sample_rate: u32) -> WavetableOscillator {
        let mut wave_table: Vec<f32> = Vec::with_capacity(WAVE_TABLE_SIZE);
        let middle: f32 = WAVE_TABLE_SIZE as f32 / 2 as f32;
        for n in 0..WAVE_TABLE_SIZE {
            let is_upper_half: f32 = (n >= (WAVE_TABLE_SIZE / 2)) as i32 as f32;
            wave_table.push(((n as f32 % (middle)) / (middle - 1 as f32)) - is_upper_half);
        }
        return WavetableOscillator::new(sample_rate, wave_table);
    }

    pub fn preset_sqr(sample_rate: u32) -> WavetableOscillator {
        let mut wave_table: Vec<f32> = Vec::with_capacity(WAVE_TABLE_SIZE);
        let middle = WAVE_TABLE_SIZE / 2;
        for n in 0..WAVE_TABLE_SIZE {
	    wave_table.push(if n < middle { 1.0 } else { -1.0 });
        }
        return WavetableOscillator::new(sample_rate, wave_table);
    }

    pub fn preset_tri(sample_rate: u32) -> WavetableOscillator {
        let mut wave_table: Vec<f32> = Vec::with_capacity(WAVE_TABLE_SIZE);
        for n in 0..WAVE_TABLE_SIZE {
	    let x: f32 = (n as f32 / WAVE_TABLE_SIZE as f32) * 2.0;
	    let wave = 1.0 - 4.0 * (0.5 - (0.5 * x + 0.25).fract()).abs();
	    wave_table.push(wave);
        }
        return WavetableOscillator::new(sample_rate, wave_table);
    }
    
    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 
                               / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();
        
        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index] 
               + next_index_weight * self.wave_table[next_index];
    }
} 

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }   

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}
