use std::f64::consts::PI;
use hound;

pub struct Wave {
    pub samples: Vec<f32>
}

impl Wave {
    const SAMPLE_RATE: u32 = 48000;
    const WAV_SPEC: hound::WavSpec = hound::WavSpec { 
        channels: 1,
        sample_rate: Wave::SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    /*
    pub fn silence(duration: f64) -> Wave {
        let sample_count: usize = (duration * Wave::SAMPLE_RATE as f64) as usize;
        Wave {
            spec: Wave::WAV_SPEC,
            samples: vec![0.0; sample_count],
        } 
    }
    */

    pub fn export(&self, export_path: &str) {
        let mut writer = hound::WavWriter::create(export_path, Wave::WAV_SPEC).unwrap();
        // TODO : Implement channel interleaving
        for sample in self.samples.iter() {
            writer.write_sample(*sample).unwrap();
        }
    }

    pub fn concat(&mut self, other_wave: &Wave) {
        self.samples.extend(&other_wave.samples)
    }
}

// Generates one full cycle containing the specified tone
pub fn gen_tone_cycle(freq: f64, amp: f64) -> Wave {
    let mut samples = Vec::new();
    // Generates a sine wave with amplitude ranging from 1.0 to 0.0
    let sample_count = (Wave::SAMPLE_RATE as f64 / freq) as u32;
    for cycle_percent in (0 .. sample_count).map(|x| x as f64 / sample_count as f64) {
        let sample = (cycle_percent * 2.0 * PI).sin() * amp;
        samples.push(sample as f32);
    }

    Wave {
        samples,
    }
}

