use std::f64::consts::PI;
use hound;

pub struct Wave {
    duration: f64,
    spec: hound::WavSpec,
    pub samples: Vec<f32>
}

impl Wave {
    pub fn silence(duration: f64, sample_rate: u32) -> Wave {
        // !!: How can this be initialized outside of this function?
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let sample_count: usize = (duration * sample_rate as f64) as usize;
        Wave {
            duration,
            spec,
            samples: vec![0.0; sample_count],
        } 
    }

    pub fn tone(freq: f64, amp: f64, duration: f64, sample_rate: u32, phase_offset: f64) -> (Wave, f64) {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let sample_count: u32 = (duration * sample_rate as f64) as u32;
        let mut samples = Vec::new();
        // Generates a sine wave with amplitude ranging from 1.0 to 0.0
        let cycle_count = freq * duration;
        for n in (0 .. sample_count).map(|x| (x as f64 / sample_count as f64) * cycle_count){
            let sample = ((n + phase_offset )* 2.0 * PI).sin() * amp;
            samples.push(sample as f32);
        }

        let final_phase = cycle_count % 1.0;
        
        (Wave {
            duration,
            spec,
            samples,
        },
        final_phase)
    }

    pub fn export(&self, export_path: &str) {
        let mut writer = hound::WavWriter::create(export_path, self.spec).unwrap();
        // TODO : Implement channel interleaving
        for sample in self.samples.iter() {
            writer.write_sample(*sample).unwrap();
        }
    }

    pub fn concat(&mut self, other_wave: &Wave) {
        self.samples.extend(&other_wave.samples)
    }
}
