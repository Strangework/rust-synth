use std::f64::consts::PI;
use std::i16;
use hound;

pub struct Wave {
    duration: f64,
    spec: hound::WavSpec,
    pub samples: Vec<i16>
}

impl Wave {
    pub fn silence(duration: f64, sample_rate: u32) -> Wave {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let sample_count: usize = (duration * sample_rate as f64) as usize;
        Wave {
            duration,
            spec,
            samples: vec![0; sample_count],
        } 
    }

    pub fn tone(freq: f64, amp: f64, duration: f64, sample_rate: u32) -> Wave {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let sample_count: usize = (duration * sample_rate as f64) as usize;
        let mut samples = Vec::new();
        for n in (0 .. sample_count).map(|x| x as f64 / sample_count as f64) {
            let sample = (n * freq * 2.0 * PI).sin() * i16::MAX as f64 * amp;
            samples.push(sample as i16);
        }
 
        Wave {
            duration,
            spec,
            samples,
        } 
    }

    pub fn export(self, export_path: &str) {
        let mut writer = hound::WavWriter::create(export_path, self.spec).unwrap();
        // TODO : Implement channel interleaving
        for sample in self.samples.iter() {
            writer.write_sample(*sample).unwrap();
        } 
    }
}
