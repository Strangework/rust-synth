mod scale;
mod wave;
use crossbeam_channel;
use std::io;
use jack;
use crate::scale::Scale;

#[derive(PartialEq)]
enum ControlSignal {
  PlayPause,
  NoteChange,
}

fn main() {
    let eq_temp = scale::EqualTemperament{
        ref_note: scale::Note {
           letter: scale::LetterNote::A,
           octave: 4,
        },
        ref_freq: 440.0,
        delta_ratio: 2f64.powf(1.0/12.0),
    };
    let c4_freq = eq_temp.get_frequency(scale::Note{
        letter: scale::LetterNote::C,
        octave: 4,
    });
    let c5_freq = eq_temp.get_frequency(scale::Note{
        letter: scale::LetterNote::C,
        octave: 5,
    });
    println!("Frequency: {}", c4_freq); 
    println!("Frequency: {}", c5_freq); 

    let (client, status) =
      jack::Client::new("tone-gen", jack::ClientOptions::NO_START_SERVER).unwrap();
    println!("{} : {:?}", client.name(), status);

    let mut port = client
      .register_port("tone-out", jack::AudioOut::default())
      .unwrap();
 
    let sample_rate = client.sample_rate() as u32;
    let (mut wav, phase_offset) = wave::Wave::tone(c4_freq, 0.8, 2.0, sample_rate, 0.0);
    let (c5_wav, phase_offset) = wave::Wave::tone(c5_freq, 0.8, 2.0, sample_rate, phase_offset);
    wav.concat(&c5_wav);
    //let wav = wave::Wave::silence(3.0, 44_100);
    println!("{:?}", wav.samples.len());
    wav.export("out.wav");

    let mut n = 0;
    let mut is_playing = false;
    let (ctrl_tx, ctrl_rx) = crossbeam_channel::bounded::<ControlSignal>(0);
    let (note_tx, note_rx) = crossbeam_channel::bounded::<scale::Note>(0);
    let process = jack::ClosureProcessHandler::new(
      move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let out = port.as_mut_slice(ps);
        for v in out.iter_mut() {
          if let Ok(sig) = ctrl_rx.try_recv() {
            /// Can I do a match here?
            if sig == ControlSignal::PlayPause {
              is_playing = !is_playing;
            } else if sig == ControlSignal::NoteChange {
              if let Ok(note) = note_rx.recv() {
                println!("HAHA! {}", note.octave);
              }
            }
          }
          if is_playing {
            *v = wav.samples[n];
            n = n % (wav.samples.len()-1) + 1;
          } else {
            *v = 0.0;
          }
        }

        jack::Control::Continue
      },
    );

    let active_client = client.activate_async((), process).unwrap();
    
    let mut buf = String::new();
    loop {
      std::io::stdin().read_line(&mut buf).unwrap();
      match buf.trim() {
        "q" => break,
        "p" => { ctrl_tx.send(ControlSignal::PlayPause); },
        _ => {
            ctrl_tx.send(ControlSignal::NoteChange);
            note_tx.send(scale::Note::from_str(buf.trim()).unwrap());
          }
      }
      buf.clear();
    }

    active_client.deactivate().unwrap(); 
}
