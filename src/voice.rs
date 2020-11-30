use crossbeam_channel::{Sender, Receiver, bounded, unbounded};
use std::thread;
use std::time::Duration;
use std::time::Instant;
use crate::wave;

#[derive(Copy, Clone, PartialEq)]
pub enum ControlOp {
    Play,
    Sustain,
}

pub struct Control {
    operation:  ControlOp,
    chunk_size: u32,
}

impl Control {
    pub fn get_op(&self) -> ControlOp {
        self.operation
    }
    
    pub fn get_chunk_size(&self) -> u32 {
        self.chunk_size
    }
}

type Conframe = (f32, Option<Control>);

pub struct Voice {
    attack:  Duration,
    decay:   Duration,
    sustain_amp: f64,
    release: Duration,
}

impl Voice {
    // !! : Defined in wave.rs as well, how can this be centrally controlled?
    const SAMPLE_RATE: u32 = 48000;
    pub fn new(attack: Duration, decay: Duration, sustain_amp: f64, release: Duration) -> Voice {
        Voice {
            attack,
            decay,
            sustain_amp,
            release,
        }
    }

    pub fn press(&self, freq: f64, amp: f64) -> (Receiver::<Conframe>, Sender::<u8>) {
        // ?? : This can be neatened up?
        let (attack, decay, sustain_amp, release) = (self.attack, self.decay, self.sustain_amp, self.release);

        let (conframe_tx, conframe_rx) = unbounded::<Conframe>();
        let (release_tx, release_rx) = unbounded::<u8>();

        thread::spawn(move || {
            let start_time = Instant::now();
            let wav = wave::gen_tone_cycle(freq, amp);
           

            // ?? : This can be it's own function?
            let mut n: usize = 0;
            let mut cycle_n: usize;
            let mut sample_count = (attack.as_secs_f64() * Voice::SAMPLE_RATE as f64) as usize;
            let mut sample;

            // Calculate attack portion
            while n < sample_count {
                cycle_n = n % (wav.samples.len()-1);
                sample = wav.samples[cycle_n] * (n as f64 / sample_count as f64) as f32;
                conframe_tx.send((sample, None)).unwrap();
                n += 1;
            }

            // Calculate decay portion
            sample_count = (decay.as_secs_f64() * Voice::SAMPLE_RATE as f64) as usize;
            // Ensures that decay calculation starts at the correct phase in the cycle
            // !! : What I am doing with the phase offset? Lookit this please
            let mut phase_offset = n % (wav.samples.len()-1);
            n = 0;
            while n < sample_count {
                cycle_n = (n + phase_offset) % (wav.samples.len()-1);
                sample = wav.samples[cycle_n] * (1.0 - (1.0 - sustain_amp) * (n as f64 / sample_count as f64)) as f32;
                conframe_tx.send((sample, None)).unwrap();
                n += 1;
            }

            // Sustain portion
            // !! : Make this accept a release signal instead of holding for an arbitrary amount of time
            /*
            sample_count = (0.5 * Voice::SAMPLE_RATE as f64) as usize;
            phase_offset = n & (wav.samples.len()-1);
            n = 0;
            while n < sample_count {
                cycle_n = (n + phase_offset) % (wav.samples.len()-1);
                sample = wav.samples[cycle_n] * sustain_amp as f32;
                conframe_tx.send(sample).unwrap();
                n += 1;
            }
            */
            let chunk_duration = Duration::new(0, 10_000_000);
            let mut next_chunk_time = start_time + attack + decay;
            loop {
                if let Ok(_x) = release_rx.try_recv() {
                    break;
                }
                if Instant::now() > next_chunk_time {
                    phase_offset += n % (wav.samples.len()-1);
                    n = 0;
                    sample_count = (chunk_duration.as_secs_f64() * Voice::SAMPLE_RATE as f64) as usize;
                    while n < sample_count {
                        cycle_n = (n + phase_offset) % (wav.samples.len()-1);
                        sample = wav.samples[cycle_n] * sustain_amp as f32;
                        conframe_tx.send((sample, None)).unwrap();
                        n += 1;
                    }
                    next_chunk_time += chunk_duration;
                }
            }

            // Calculate release portion
            sample_count = (release.as_secs_f64() * Voice::SAMPLE_RATE as f64) as usize;
            phase_offset += n % (wav.samples.len()-1);
            n = 0;
            while n < sample_count {
                cycle_n = (n + phase_offset) % (wav.samples.len()-1);
                sample = wav.samples[cycle_n] * (sustain_amp * (1.0 - (n as f64 / sample_count as f64))) as f32;
                conframe_tx.send((sample, None)).unwrap();
                n += 1;
            }
        });

        (conframe_rx, release_tx)
    }
}
