use std::f64::consts::PI;
use hound;

pub trait SampleGen {
    fn get_frames(&mut self, n: usize) -> &[f32];
    fn get_full_cycle(&self) -> Vec<f32>;
}

pub struct SineGen {
    samples: Vec<f32>,
    sample_index: usize,
}

impl SineGen {
    const SAMPLE_RATE: u32 = 48000;
    const WAV_SPEC: hound::WavSpec = hound::WavSpec { 
        channels: 1,
        sample_rate: SineGen::SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    // Generates one full cycle containing the specified tone
    pub fn new(freq: f64, amp: f64) -> SineGen {
        let mut samples = Vec::new();
        // Generates a sine wave with amplitude ranging from 1.0 to 0.0
        let sample_count = (SineGen::SAMPLE_RATE as f64 / freq) as u32;
        for cycle_percent in (0 .. sample_count).map(|x| x as f64 / sample_count as f64) {
            let sample = (cycle_percent * 2.0 * PI).sin() * amp;
            samples.push(sample as f32);
        }

        SineGen {
            samples,
            sample_index: 0
        }
    }


    // !! : Should be moved to some class for manipulating &[f32]
    pub fn export(&self, export_path: &str) {
        let mut writer = hound::WavWriter::create(export_path, SineGen::WAV_SPEC).unwrap();
        // TODO : Implement channel interleaving
        for sample in self.samples.iter() {
            writer.write_sample(*sample).unwrap();
        }
    }

    // !! : Should be moved to some class for manipulating &[f32]
    pub fn concat(&mut self, other_wave: &SineGen) {
        self.samples.extend(&other_wave.samples)
    }
}

impl SampleGen for SineGen {
    fn get_frames(&mut self, n: usize) -> &[f32] {
        let new_index = (self.sample_index + n)  % (self.samples.len()-1);
        let frames = &self.samples[self.sample_index..new_index];
        self.sample_index = new_index;

        frames
    }
    
    fn get_full_cycle(&self) -> Vec<f32> {
        self.samples.clone()
    }
}
